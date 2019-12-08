#[derive(Default)]
pub struct SandboxListener {
    pub on_command_started: Option<Box<dyn Fn(String) + Send + Sync>>,
    pub on_command_stdout: Option<Box<dyn Fn(String) + Send + Sync>>,
    pub on_command_stderr: Option<Box<dyn Fn(String) + Send + Sync>>,
}