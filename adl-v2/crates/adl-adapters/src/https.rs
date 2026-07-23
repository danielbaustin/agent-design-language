use reqwest::{Client, Method, StatusCode};
use secrecy::{ExposeSecret, SecretString};
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time::Instant;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpAdapterError {
    InvalidPermit,
    PermitDenied,
    PermitMismatch,
    InvalidMethod,
    OversizedRequest,
    OversizedResponse,
    Authentication,
    RateLimited,
    Redirected,
    Rejected(u16),
    Unavailable,
    Timeout,
    Cancelled,
    Malformed,
}

/// Injected endpoint authority. Parsing a URL is never sufficient authority.
pub trait EndpointAuthorizer {
    fn authorize(&mut self, endpoint: &Url) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointPermit {
    endpoint: Url,
    local_test_http: bool,
}

impl EndpointPermit {
    pub fn admit<A: EndpointAuthorizer>(
        endpoint: &str,
        authorizer: &mut A,
    ) -> Result<Self, HttpAdapterError> {
        let endpoint = validate_endpoint(endpoint)?;
        if !authorizer.authorize(&endpoint) {
            return Err(HttpAdapterError::PermitDenied);
        }
        Ok(Self {
            endpoint,
            local_test_http: false,
        })
    }

    /// Deterministic loopback-only transport proof. Absent from default builds.
    #[cfg(feature = "test-transport")]
    pub fn admit_local_test<A: EndpointAuthorizer>(
        endpoint: &str,
        authorizer: &mut A,
    ) -> Result<Self, HttpAdapterError> {
        let endpoint = validate_local_test_endpoint(endpoint)?;
        if !authorizer.authorize(&endpoint) {
            return Err(HttpAdapterError::PermitDenied);
        }
        Ok(Self {
            endpoint,
            local_test_http: true,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CancellationHandle(watch::Sender<bool>);

#[derive(Debug, Clone)]
pub struct Cancellation(watch::Receiver<bool>);

pub fn cancellation_pair() -> (CancellationHandle, Cancellation) {
    let (sender, receiver) = watch::channel(false);
    (CancellationHandle(sender), Cancellation(receiver))
}

impl CancellationHandle {
    pub fn cancel(&self) {
        let _ = self.0.send(true);
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub endpoint: String,
    pub body: Vec<u8>,
    pub bearer: Option<SecretString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Clone)]
pub struct HttpAdapter {
    client: Client,
    max_request_bytes: usize,
    max_response_bytes: usize,
    deadline: Duration,
}

impl HttpAdapter {
    pub fn new(
        max_request_bytes: usize,
        max_response_bytes: usize,
        deadline: Duration,
    ) -> Result<Self, HttpAdapterError> {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .connect_timeout(deadline)
            .build()
            .map_err(|_| HttpAdapterError::Malformed)?;
        Ok(Self {
            client,
            max_request_bytes,
            max_response_bytes,
            deadline,
        })
    }

    pub async fn execute(
        &self,
        permit: &EndpointPermit,
        request: HttpRequest,
        cancellation: &Cancellation,
    ) -> Result<HttpResponse, HttpAdapterError> {
        if request.body.len() > self.max_request_bytes {
            return Err(HttpAdapterError::OversizedRequest);
        }
        let endpoint = if permit.local_test_http {
            #[cfg(feature = "test-transport")]
            {
                validate_local_test_endpoint(&request.endpoint)
            }
            #[cfg(not(feature = "test-transport"))]
            {
                Err(HttpAdapterError::PermitMismatch)
            }
        } else {
            validate_endpoint(&request.endpoint)
        }
        .map_err(|_| HttpAdapterError::PermitMismatch)?;
        if endpoint != permit.endpoint {
            return Err(HttpAdapterError::PermitMismatch);
        }
        let method = Method::from_bytes(request.method.as_bytes())
            .map_err(|_| HttpAdapterError::InvalidMethod)?;
        let bearer = request.bearer;
        let mut builder = self.client.request(method, endpoint).body(request.body);
        if let Some(value) = &bearer {
            builder = builder.bearer_auth(value.expose_secret());
        }
        let deadline = Instant::now() + self.deadline;
        let mut cancelled = cancellation.0.clone();
        let mut response = tokio::select! {
            result = tokio::time::timeout_at(deadline, builder.send()) => {
                result.map_err(|_| HttpAdapterError::Timeout)?.map_err(classify_transport)?
            }
            _ = await_cancellation(&mut cancelled) => return Err(HttpAdapterError::Cancelled),
        };
        let status = response.status();
        classify_status(status)?;
        if response
            .content_length()
            .is_some_and(|n| n > self.max_response_bytes as u64)
        {
            return Err(HttpAdapterError::OversizedResponse);
        }
        let exposed = bearer.as_ref().map(ExposeSecret::expose_secret);
        let headers = safe_headers(response.headers(), exposed);
        let mut body = Vec::new();
        loop {
            let chunk = tokio::select! {
                result = tokio::time::timeout_at(deadline, response.chunk()) => {
                    result.map_err(|_| HttpAdapterError::Timeout)?.map_err(classify_transport)?
                }
                _ = await_cancellation(&mut cancelled) => return Err(HttpAdapterError::Cancelled),
            };
            let Some(chunk) = chunk else { break };
            if body.len().saturating_add(chunk.len()) > self.max_response_bytes {
                return Err(HttpAdapterError::OversizedResponse);
            }
            body.extend_from_slice(&chunk);
        }
        if exposed.is_some_and(|value| {
            !value.is_empty()
                && body
                    .windows(value.len())
                    .any(|part| part == value.as_bytes())
        }) {
            return Err(HttpAdapterError::Malformed);
        }
        Ok(HttpResponse {
            status: status.as_u16(),
            headers,
            body,
        })
    }
}

fn validate_endpoint(endpoint: &str) -> Result<Url, HttpAdapterError> {
    let endpoint = Url::parse(endpoint).map_err(|_| HttpAdapterError::InvalidPermit)?;
    if endpoint.scheme() != "https"
        || !endpoint.username().is_empty()
        || endpoint.password().is_some()
        || endpoint.fragment().is_some()
    {
        return Err(HttpAdapterError::InvalidPermit);
    }
    Ok(endpoint)
}

#[cfg(feature = "test-transport")]
fn validate_local_test_endpoint(endpoint: &str) -> Result<Url, HttpAdapterError> {
    let endpoint = Url::parse(endpoint).map_err(|_| HttpAdapterError::InvalidPermit)?;
    let loopback = endpoint
        .host_str()
        .and_then(|host| host.parse::<std::net::IpAddr>().ok())
        .is_some_and(|address| address.is_loopback());
    if endpoint.scheme() != "http"
        || !loopback
        || !endpoint.username().is_empty()
        || endpoint.password().is_some()
        || endpoint.fragment().is_some()
    {
        return Err(HttpAdapterError::InvalidPermit);
    }
    Ok(endpoint)
}

fn classify_status(status: StatusCode) -> Result<(), HttpAdapterError> {
    match status.as_u16() {
        200..=299 => Ok(()),
        300..=399 => Err(HttpAdapterError::Redirected),
        401 | 403 => Err(HttpAdapterError::Authentication),
        429 => Err(HttpAdapterError::RateLimited),
        500..=599 => Err(HttpAdapterError::Unavailable),
        value => Err(HttpAdapterError::Rejected(value)),
    }
}

fn safe_headers(
    headers: &reqwest::header::HeaderMap,
    secret: Option<&str>,
) -> BTreeMap<String, String> {
    ["content-type", "retry-after", "x-request-id"]
        .into_iter()
        .filter_map(|name| {
            let value = headers.get(name)?.to_str().ok()?;
            let safe = value.len() <= 1024
                && !secret.is_some_and(|secret| !secret.is_empty() && value.contains(secret));
            safe.then(|| (name.to_owned(), value.to_owned()))
        })
        .collect()
}

fn classify_transport(error: reqwest::Error) -> HttpAdapterError {
    if error.is_timeout() {
        HttpAdapterError::Timeout
    } else if error.is_connect() {
        HttpAdapterError::Unavailable
    } else {
        HttpAdapterError::Malformed
    }
}

async fn await_cancellation(receiver: &mut watch::Receiver<bool>) {
    loop {
        if *receiver.borrow() {
            return;
        }
        if receiver.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}
