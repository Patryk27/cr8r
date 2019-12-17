use crate::{Result, SandboxListener, ShellEngine};

pub async fn init(engine: &mut ShellEngine, listener: SandboxListener) -> Result<()> {
    engine.listener = listener;

    // @todo ensure `dir` exists

    Ok(())
}