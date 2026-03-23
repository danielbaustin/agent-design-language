pub struct PlanHeaders<'a> {
    pub run: &'a str,
    pub workflow: &'a str,
    pub steps: &'a str,
}

pub fn print_plan<I, F>(
    headers: PlanHeaders<'_>,
    run_id: &str,
    workflow_id: &str,
    step_count: usize,
    steps: I,
    mut format_step: F,
) where
    I: IntoIterator,
    F: FnMut(I::Item) -> String,
{
    println!("{} {}", headers.run, run_id);
    println!("{} {}", headers.workflow, workflow_id);
    println!("{} {}", headers.steps, step_count);

    for (idx, step) in steps.into_iter().enumerate() {
        println!("  {idx}. {}", format_step(step));
    }
}
