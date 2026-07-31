use adl_adapters::{
    cancellation_pair, EndpointAuthorizer, EndpointPermit, HttpAdapter, HttpAdapterError,
    HttpRequest,
};
use std::time::Duration;
use url::Url;
use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

struct Authority(bool);
impl EndpointAuthorizer for Authority {
    fn authorize(&mut self, _: &Url) -> bool {
        self.0
    }
}
fn permit(endpoint: &str) -> Result<EndpointPermit, HttpAdapterError> {
    EndpointPermit::admit(endpoint, &mut Authority(true))
}
fn adapter() -> HttpAdapter {
    HttpAdapter::new(4, 8, Duration::from_millis(50)).unwrap()
}
fn request(endpoint: &str) -> HttpRequest {
    HttpRequest {
        method: "POST".into(),
        endpoint: endpoint.into(),
        body: vec![],
        bearer: None,
    }
}
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn local_permit(endpoint: &str) -> EndpointPermit {
    EndpointPermit::admit_local_test(endpoint, &mut Authority(true)).unwrap()
}

#[test]
fn https_permit_is_accepted() {
    assert!(permit("https://example.com/v1").is_ok());
}
#[test]
fn authority_can_deny_valid_url() {
    assert_eq!(
        EndpointPermit::admit("https://example.com", &mut Authority(false)),
        Err(HttpAdapterError::PermitDenied)
    );
}
#[test]
fn http_permit_is_rejected() {
    assert_eq!(
        permit("http://example.com"),
        Err(HttpAdapterError::InvalidPermit)
    );
}
#[test]
fn userinfo_is_rejected() {
    assert_eq!(
        permit("https://u:p@example.com"),
        Err(HttpAdapterError::InvalidPermit)
    );
}
#[test]
fn fragment_is_rejected() {
    assert_eq!(
        permit("https://example.com/#x"),
        Err(HttpAdapterError::InvalidPermit)
    );
}
#[test]
fn malformed_url_is_rejected() {
    assert_eq!(permit("not a url"), Err(HttpAdapterError::InvalidPermit));
}
#[test]
fn oversized_request_is_rejected_before_io() {
    let p = permit("https://example.com").unwrap();
    let mut r = request("https://example.com");
    r.body = vec![0; 5];
    let (_, c) = cancellation_pair();
    assert_eq!(
        runtime().block_on(adapter().execute(&p, r, &c)),
        Err(HttpAdapterError::OversizedRequest)
    );
}
#[test]
fn endpoint_mismatch_is_rejected_before_io() {
    let p = permit("https://example.com/a").unwrap();
    let (_, c) = cancellation_pair();
    assert_eq!(
        runtime().block_on(adapter().execute(&p, request("https://example.com/b"), &c)),
        Err(HttpAdapterError::PermitMismatch)
    );
}
#[test]
fn invalid_method_is_rejected_before_io() {
    let p = permit("https://example.com").unwrap();
    let mut r = request("https://example.com");
    r.method = "bad method".into();
    let (_, c) = cancellation_pair();
    assert_eq!(
        runtime().block_on(adapter().execute(&p, r, &c)),
        Err(HttpAdapterError::InvalidMethod)
    );
}
#[test]
fn cancellation_is_distinct() {
    let p = permit("https://example.com").unwrap();
    let (h, c) = cancellation_pair();
    h.cancel();
    assert_eq!(
        runtime().block_on(adapter().execute(&p, request("https://example.com"), &c)),
        Err(HttpAdapterError::Cancelled)
    );
}

#[test]
fn wire_response_is_bounded_and_headers_are_allowlisted() {
    runtime().block_on(async {
        let server = MockServer::start().await;
        let endpoint = format!("{}/v1", server.uri());
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"ok")
                    .insert_header("content-type", "text/plain")
                    .insert_header("set-cookie", "secret=cookie"),
            )
            .mount(&server)
            .await;
        let (_, cancellation) = cancellation_pair();
        let response = adapter()
            .execute(&local_permit(&endpoint), request(&endpoint), &cancellation)
            .await
            .unwrap();
        assert_eq!(response.body, b"ok");
        assert_eq!(response.headers.get("content-type").unwrap(), "text/plain");
        assert!(!response.headers.contains_key("set-cookie"));
    });
}

#[test]
fn wire_response_content_length_is_capped() {
    runtime().block_on(async {
        let server = MockServer::start().await;
        let endpoint = format!("{}/v1", server.uri());
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0; 9]))
            .mount(&server)
            .await;
        let (_, cancellation) = cancellation_pair();
        assert_eq!(
            adapter()
                .execute(&local_permit(&endpoint), request(&endpoint), &cancellation)
                .await,
            Err(HttpAdapterError::OversizedResponse)
        );
    });
}

#[test]
fn wire_redirect_is_not_followed() {
    assert_wire_status(302, HttpAdapterError::Redirected);
}
#[test]
fn wire_authentication_is_distinct() {
    assert_wire_status(401, HttpAdapterError::Authentication);
}
#[test]
fn wire_rate_limit_is_distinct() {
    assert_wire_status(429, HttpAdapterError::RateLimited);
}
#[test]
fn wire_unavailable_is_distinct() {
    assert_wire_status(503, HttpAdapterError::Unavailable);
}
#[test]
fn wire_rejection_preserves_status() {
    assert_wire_status(422, HttpAdapterError::Rejected(422));
}

fn assert_wire_status(status: u16, expected: HttpAdapterError) {
    runtime().block_on(async {
        let server = MockServer::start().await;
        let endpoint = format!("{}/v1", server.uri());
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(status))
            .mount(&server)
            .await;
        let (_, cancellation) = cancellation_pair();
        assert_eq!(
            adapter()
                .execute(&local_permit(&endpoint), request(&endpoint), &cancellation)
                .await,
            Err(expected)
        );
    });
}

#[test]
fn wire_deadline_covers_response_body() {
    runtime().block_on(async {
        let server = MockServer::start().await;
        let endpoint = format!("{}/v1", server.uri());
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(100)))
            .mount(&server)
            .await;
        let (_, cancellation) = cancellation_pair();
        let short = HttpAdapter::new(4, 8, Duration::from_millis(10)).unwrap();
        assert_eq!(
            short
                .execute(&local_permit(&endpoint), request(&endpoint), &cancellation)
                .await,
            Err(HttpAdapterError::Timeout)
        );
    });
}

#[test]
fn wire_echoed_secret_is_rejected_from_body_and_headers() {
    runtime().block_on(async {
        let server = MockServer::start().await;
        let endpoint = format!("{}/v1", server.uri());
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"canary-secret")
                    .insert_header("x-request-id", "canary-secret"),
            )
            .mount(&server)
            .await;
        let (_, cancellation) = cancellation_pair();
        let mut req = request(&endpoint);
        req.bearer = Some(secrecy::SecretString::from("canary-secret"));
        assert_eq!(
            HttpAdapter::new(4, 32, Duration::from_millis(50))
                .unwrap()
                .execute(&local_permit(&endpoint), req, &cancellation)
                .await,
            Err(HttpAdapterError::Malformed)
        );
    });
}
