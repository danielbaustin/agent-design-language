use crate::obsmem_contract::{
    MemoryQuery, MemoryQueryResult, MemoryRecord, ObsMemContractError, ObsMemContractErrorCode,
    OBSMEM_CONTRACT_VERSION,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrievalOrder {
    /// Canonical default: descending explicit score, then deterministic lexical tie-breaks.
    ///
    /// v0.8 note: this is deterministic ranking over scores already present on
    /// retrieved records. It does not perform hidden Bayesian updates or infer
    /// new confidence values from memory hits.
    ScoreDescIdAsc,
    /// Bounded evidence-adjusted ranking using explicit observed tags/citations.
    ///
    /// This is a deterministic, reviewable Bayes-style update surface for v0.8:
    /// the stored score is treated as the prior and explicit evidence on the
    /// retrieved record applies a bounded multiplicative adjustment.
    EvidenceAdjustedDescIdAsc,
    /// Simple deterministic lexical ordering for explicit policy/testing.
    IdAsc,
}

/// Deterministic retrieval policy contract for v0.75 structured retrieval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievalPolicyV1 {
    pub default_limit: usize,
    pub required_tags: Vec<String>,
    pub required_failure_code: Option<String>,
    pub order: RetrievalOrder,
}

impl Default for RetrievalPolicyV1 {
    fn default() -> Self {
        Self {
            default_limit: 10,
            required_tags: Vec::new(),
            required_failure_code: None,
            order: RetrievalOrder::ScoreDescIdAsc,
        }
    }
}

impl RetrievalPolicyV1 {
    /// Canonicalize policy state for deterministic comparisons/application.
    pub fn normalize(&mut self) {
        self.required_tags.sort();
        self.required_tags.dedup();
    }

    /// Validate policy bounds before building/processing queries.
    pub fn validate(&self) -> Result<(), ObsMemContractError> {
        if self.default_limit == 0 || self.default_limit > 1000 {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidQuery,
                "retrieval policy default_limit must be in 1..=1000",
            ));
        }
        Ok(())
    }
}

/// Caller-provided retrieval request prior to policy normalization/merging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievalRequest {
    pub workflow_id: Option<String>,
    pub failure_code: Option<String>,
    pub tags: Vec<String>,
    pub limit_override: Option<usize>,
}

impl RetrievalRequest {
    /// Canonicalize request tags for deterministic behavior.
    pub fn normalize(&mut self) {
        self.tags.sort();
        self.tags.dedup();
    }

    /// Convert a retrieval request plus policy into a validated contract query.
    pub fn to_query(&self, policy: &RetrievalPolicyV1) -> Result<MemoryQuery, ObsMemContractError> {
        policy.validate()?;
        let mut request = self.clone();
        request.normalize();

        let mut merged_tags = request.tags;
        merged_tags.extend(policy.required_tags.iter().cloned());
        merged_tags.sort();
        merged_tags.dedup();

        let limit = request.limit_override.unwrap_or(policy.default_limit);
        if limit == 0 || limit > 1000 {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidQuery,
                "retrieval request limit must be in 1..=1000",
            ));
        }

        let failure_code = request
            .failure_code
            .or_else(|| policy.required_failure_code.clone());

        let mut q = MemoryQuery {
            contract_version: OBSMEM_CONTRACT_VERSION,
            workflow_id: request.workflow_id,
            failure_code,
            tags: merged_tags,
            limit,
        };
        q.normalize();
        q.validate()?;
        Ok(q)
    }
}

/// Apply policy-level filtering/order/limits to backend query results.
///
/// For identical inputs and index state, this function must produce identical
/// result ordering and truncation.
///
/// v0.8 boundary: the policy layer filters and sorts explicit retrieval
/// results, but it does not synthesize posterior confidence or mutate scores.
pub fn apply_policy_to_results(
    policy: &RetrievalPolicyV1,
    request: &RetrievalRequest,
    mut result: MemoryQueryResult,
) -> Result<MemoryQueryResult, ObsMemContractError> {
    policy.validate()?;

    let query = request.to_query(policy)?;

    let required_failure_tag = query
        .failure_code
        .as_ref()
        .map(|fc| format!("failure:{fc}"));

    result.hits.retain(|hit| {
        query
            .workflow_id
            .as_ref()
            .is_none_or(|wid| wid == &hit.workflow_id)
            && query.tags.iter().all(|t| hit.tags.binary_search(t).is_ok())
            && required_failure_tag
                .as_ref()
                .is_none_or(|tag| hit.tags.binary_search(tag).is_ok())
    });

    stable_sort_hits(policy.order, &query, &mut result.hits);
    result.hits.truncate(query.limit);
    Ok(result)
}

fn stable_sort_hits(order: RetrievalOrder, query: &MemoryQuery, hits: &mut [MemoryRecord]) {
    match order {
        RetrievalOrder::ScoreDescIdAsc => {
            hits.sort_by(|a, b| {
                parse_score_hundredths(&b.score)
                    .cmp(&parse_score_hundredths(&a.score))
                    .then_with(|| a.id.cmp(&b.id))
                    .then_with(|| a.run_id.cmp(&b.run_id))
                    .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                    .then_with(|| a.payload.cmp(&b.payload))
            });
        }
        RetrievalOrder::EvidenceAdjustedDescIdAsc => {
            hits.sort_by(|a, b| {
                evidence_adjusted_score_hundredths(b, query)
                    .cmp(&evidence_adjusted_score_hundredths(a, query))
                    .then_with(|| {
                        parse_score_hundredths(&b.score).cmp(&parse_score_hundredths(&a.score))
                    })
                    .then_with(|| a.id.cmp(&b.id))
                    .then_with(|| a.run_id.cmp(&b.run_id))
                    .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                    .then_with(|| a.payload.cmp(&b.payload))
            });
        }
        RetrievalOrder::IdAsc => {
            hits.sort_by(|a, b| {
                a.id.cmp(&b.id)
                    .then_with(|| a.run_id.cmp(&b.run_id))
                    .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                    .then_with(|| a.payload.cmp(&b.payload))
            });
        }
    }
}

fn evidence_adjusted_score_hundredths(hit: &MemoryRecord, query: &MemoryQuery) -> i64 {
    let prior = parse_score_hundredths(&hit.score).max(0);
    let multiplier_percent = evidence_multiplier_percent(hit, query);
    ((prior + 100) * multiplier_percent) / 100
}

fn evidence_multiplier_percent(hit: &MemoryRecord, query: &MemoryQuery) -> i64 {
    let mut percent = 100;

    if has_tag(hit, "status:success") {
        percent += 35;
    }
    if has_tag(hit, "status:failed") || has_tag(hit, "status:failure") {
        percent -= 15;
    }

    let shared_query_tags = query.tags.iter().filter(|tag| has_tag(hit, tag)).count() as i64;
    percent += shared_query_tags.min(4) * 5;

    let citation_boost = (hit.citations.len() as i64).min(3) * 5;
    percent += citation_boost;

    percent.clamp(25, 200)
}

fn has_tag(hit: &MemoryRecord, tag: &str) -> bool {
    hit.tags
        .binary_search_by(|existing| existing.as_str().cmp(tag))
        .is_ok()
}

fn parse_score_hundredths(score: &str) -> i64 {
    let trimmed = score.trim();
    let mut iter = trimmed.split('.');
    let whole = iter.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let frac_raw = iter.next().unwrap_or("0");
    let frac_two = match frac_raw.len() {
        0 => "00".to_string(),
        1 => format!("{}0", frac_raw),
        _ => frac_raw[..2].to_string(),
    };
    let frac = frac_two.parse::<i64>().unwrap_or(0);
    whole * 100 + frac
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::obsmem_contract::{MemoryCitation, MemoryQueryResult, MemoryRecord};

    fn hit(id: &str, score: &str, tags: &[&str]) -> MemoryRecord {
        let mut t: Vec<String> = tags.iter().map(|s| (*s).to_string()).collect();
        t.sort();
        t.dedup();
        MemoryRecord {
            id: id.to_string(),
            run_id: format!("run-{id}"),
            workflow_id: "wf-a".to_string(),
            tags: t,
            payload: format!("payload-{id}"),
            score: score.to_string(),
            citations: vec![MemoryCitation {
                path: format!("runs/{id}/run_summary.json"),
                hash: "det64:0000000000000001".to_string(),
            }],
        }
    }

    #[test]
    fn to_query_is_deterministic_for_identical_inputs() {
        let mut policy = RetrievalPolicyV1 {
            default_limit: 5,
            required_tags: vec!["status:success".to_string(), "status:success".to_string()],
            required_failure_code: Some("tool_failure".to_string()),
            order: RetrievalOrder::ScoreDescIdAsc,
        };
        policy.normalize();

        let mut request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec!["workflow:wf-a".to_string(), "workflow:wf-a".to_string()],
            limit_override: Some(3),
        };
        request.normalize();

        let left = request.to_query(&policy).expect("left query");
        let right = request.to_query(&policy).expect("right query");
        assert_eq!(left, right);
        assert_eq!(left.failure_code.as_deref(), Some("tool_failure"));
        assert_eq!(left.tags, vec!["status:success", "workflow:wf-a"]);
        assert_eq!(left.limit, 3);
    }

    #[test]
    fn apply_policy_filters_and_orders_deterministically() {
        let mut policy = RetrievalPolicyV1 {
            default_limit: 2,
            required_tags: vec!["status:success".to_string()],
            required_failure_code: Some("tool_failure".to_string()),
            order: RetrievalOrder::ScoreDescIdAsc,
        };
        policy.normalize();

        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec!["workflow:wf-a".to_string()],
            limit_override: None,
        };

        let input = MemoryQueryResult {
            hits: vec![
                hit(
                    "b",
                    "1.20",
                    &["workflow:wf-a", "status:success", "failure:tool_failure"],
                ),
                hit(
                    "a",
                    "1.20",
                    &["workflow:wf-a", "status:success", "failure:tool_failure"],
                ),
                hit(
                    "c",
                    "9.99",
                    &["workflow:wf-a", "status:success", "failure:runtime_failure"],
                ),
            ],
        };

        let left = apply_policy_to_results(&policy, &request, input.clone()).expect("left");
        let right = apply_policy_to_results(&policy, &request, input).expect("right");

        assert_eq!(left, right);
        assert_eq!(left.hits.len(), 2);
        assert_eq!(left.hits[0].id, "a");
        assert_eq!(left.hits[1].id, "b");
    }

    #[test]
    fn apply_policy_uses_explicit_scores_without_hidden_confidence_updates() {
        let policy = RetrievalPolicyV1 {
            default_limit: 3,
            required_tags: vec![],
            required_failure_code: None,
            order: RetrievalOrder::ScoreDescIdAsc,
        };

        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec![],
            limit_override: None,
        };

        let input = MemoryQueryResult {
            hits: vec![
                hit("b", "0.25", &[]),
                hit("a", "0.25", &[]),
                hit("c", "0.90", &[]),
            ],
        };

        let output = apply_policy_to_results(&policy, &request, input).expect("apply policy");
        let ids: Vec<&str> = output.hits.iter().map(|h| h.id.as_str()).collect();
        let scores: Vec<&str> = output.hits.iter().map(|h| h.score.as_str()).collect();

        assert_eq!(ids, vec!["c", "a", "b"]);
        assert_eq!(scores, vec!["0.90", "0.25", "0.25"]);
    }

    #[test]
    fn policy_and_request_validation_reject_invalid_limits() {
        let policy = RetrievalPolicyV1 {
            default_limit: 0,
            required_tags: vec![],
            required_failure_code: None,
            order: RetrievalOrder::ScoreDescIdAsc,
        };
        let err = policy.validate().expect_err("default limit 0 must fail");
        assert!(err.message.contains("default_limit must be in 1..=1000"));

        let policy = RetrievalPolicyV1::default();
        let request = RetrievalRequest {
            workflow_id: None,
            failure_code: None,
            tags: vec![],
            limit_override: Some(1001),
        };
        let err = request
            .to_query(&policy)
            .expect_err("limit override above max must fail");
        assert!(err.message.contains("limit must be in 1..=1000"));
    }

    #[test]
    fn apply_policy_supports_id_ascending_order_and_parses_score_edges() {
        let mut policy = RetrievalPolicyV1 {
            default_limit: 10,
            required_tags: vec![],
            required_failure_code: None,
            order: RetrievalOrder::IdAsc,
        };
        policy.normalize();
        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec![],
            limit_override: None,
        };
        let input = MemoryQueryResult {
            hits: vec![
                hit("c", "0", &[]),
                hit("a", "7.1", &[]),
                hit("b", "7.05", &[]),
            ],
        };
        let out = apply_policy_to_results(&policy, &request, input).expect("policy result");
        assert_eq!(
            out.hits.iter().map(|h| h.id.as_str()).collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );

        // Exercise numeric parsing edge behavior used by score ordering.
        assert_eq!(parse_score_hundredths("7"), 700);
        assert_eq!(parse_score_hundredths("7.1"), 710);
        assert_eq!(parse_score_hundredths("7.05"), 705);
        assert_eq!(parse_score_hundredths("bad"), 0);
    }

    #[test]
    fn evidence_adjusted_order_uses_explicit_status_and_citations() {
        let mut policy = RetrievalPolicyV1 {
            default_limit: 10,
            required_tags: vec![],
            required_failure_code: None,
            order: RetrievalOrder::EvidenceAdjustedDescIdAsc,
        };
        policy.normalize();

        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec!["workflow:wf-a".to_string()],
            limit_override: None,
        };

        let mut strongest = hit("c", "0.80", &["workflow:wf-a", "status:success"]);
        strongest.citations.push(MemoryCitation {
            path: "runs/c/activation_log.json".to_string(),
            hash: "det64:0000000000000002".to_string(),
        });
        strongest.citations.push(MemoryCitation {
            path: "runs/c/run_status.json".to_string(),
            hash: "det64:0000000000000003".to_string(),
        });

        let input = MemoryQueryResult {
            hits: vec![
                hit("b", "1.00", &["workflow:wf-a", "status:failed"]),
                hit("a", "1.00", &["workflow:wf-a"]),
                strongest,
            ],
        };

        let out = apply_policy_to_results(&policy, &request, input).expect("policy result");
        assert_eq!(
            out.hits.iter().map(|h| h.id.as_str()).collect::<Vec<_>>(),
            vec!["c", "a", "b"]
        );
    }

    #[test]
    fn evidence_adjusted_order_is_deterministic_for_identical_inputs() {
        let policy = RetrievalPolicyV1 {
            default_limit: 10,
            required_tags: vec![],
            required_failure_code: None,
            order: RetrievalOrder::EvidenceAdjustedDescIdAsc,
        };
        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec!["workflow:wf-a".to_string()],
            limit_override: None,
        };
        let input = MemoryQueryResult {
            hits: vec![
                hit("b", "1.00", &["workflow:wf-a", "status:success"]),
                hit("a", "1.00", &["workflow:wf-a", "status:success"]),
            ],
        };

        let left = apply_policy_to_results(&policy, &request, input.clone()).expect("left");
        let right = apply_policy_to_results(&policy, &request, input).expect("right");
        assert_eq!(left, right);
        assert_eq!(
            left.hits.iter().map(|h| h.id.as_str()).collect::<Vec<_>>(),
            vec!["a", "b"]
        );
    }

    #[test]
    fn apply_policy_allows_empty_filters_and_returns_stable_empty_result() {
        let policy = RetrievalPolicyV1::default();
        let request = RetrievalRequest {
            workflow_id: None,
            failure_code: None,
            tags: vec![],
            limit_override: Some(1),
        };
        let result = MemoryQueryResult { hits: vec![] };
        let left = apply_policy_to_results(&policy, &request, result.clone())
            .expect("empty filters are valid");
        let right =
            apply_policy_to_results(&policy, &request, result).expect("empty filters are valid");
        assert_eq!(left, right);
        assert!(left.hits.is_empty());
    }
}
