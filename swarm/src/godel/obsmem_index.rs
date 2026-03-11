use serde::{Deserialize, Serialize};

use super::experiment_record::StageExperimentRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageIndexEntry {
    pub index_key: String,
    pub run_id: String,
    pub workflow_id: String,
    pub failure_code: String,
    pub hypothesis_id: String,
    pub evaluation_decision: String,
}

pub fn build_index_entry(record: &StageExperimentRecord, failure_code: &str) -> StageIndexEntry {
    StageIndexEntry {
        index_key: format!(
            "{}:{}:{}",
            record.workflow_id, failure_code, record.hypothesis_id
        ),
        run_id: record.run_id.clone(),
        workflow_id: record.workflow_id.clone(),
        failure_code: failure_code.to_string(),
        hypothesis_id: record.hypothesis_id.clone(),
        evaluation_decision: record.evaluation_decision.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::experiment_record::StageExperimentRecord;

    #[test]
    fn build_index_entry_has_stable_key() {
        let record = StageExperimentRecord {
            run_id: "r1".to_string(),
            workflow_id: "wf1".to_string(),
            hypothesis_id: "hyp:r1:tool_failure".to_string(),
            mutation_id: "mut:r1:tool_failure".to_string(),
            evaluation_decision: "adopt".to_string(),
            improvement_delta: 1,
        };
        let entry = build_index_entry(&record, "tool_failure");
        assert_eq!(entry.index_key, "wf1:tool_failure:hyp:r1:tool_failure");
    }
}
