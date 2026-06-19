use anyhow::{Context, Result};

pub(crate) fn build_current_thread_runtime(context: &str) -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .with_context(|| context.to_string())
}

pub(crate) fn with_current_thread_runtime<T>(
    context: &str,
    f: impl FnOnce(&tokio::runtime::Runtime) -> Result<T>,
) -> Result<T> {
    let runtime = build_current_thread_runtime(context)?;
    let _runtime_guard = runtime.enter();
    f(&runtime)
}
