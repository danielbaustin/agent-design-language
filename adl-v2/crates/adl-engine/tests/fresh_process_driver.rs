mod common;

use adl_engine::{Engine, FailureClass, TurnInput};

fn main() {
    let plan = common::plan(&["node-b", "node-a"], &[]);
    let policy = common::retry_policy(&plan, 2, 2);
    let mut engine = Engine::new(plan, policy, common::limits()).unwrap();
    let first = engine.turn(TurnInput::tick(1)).unwrap();
    let mut requests = common::provider_requests(&first);
    requests.sort_by(|left, right| left.node_id.cmp(&right.node_id));
    let retry = common::provider_failure(&requests[0], FailureClass::Retryable);
    let success = common::provider_success(&requests[1], b"second");
    engine
        .turn(TurnInput {
            logical_tick: 2,
            completions: vec![success, retry],
            cancellations: vec![],
        })
        .unwrap();
    let retry_request = common::provider_request(&engine.turn(TurnInput::tick(4)).unwrap());
    engine
        .turn(TurnInput {
            logical_tick: 5,
            completions: vec![common::provider_success(&retry_request, b"first")],
            cancellations: vec![],
        })
        .unwrap();
    println!("{}", serde_json::to_string(engine.snapshot()).unwrap());
}
