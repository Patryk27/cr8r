#[derive(Default)]
pub struct SandboxListener {
    pub on_command_executed: Option<Box<dyn Fn(String) + Send + Sync>>,
    pub on_command_output: Option<Box<dyn Fn(String) + Send + Sync>>,
}