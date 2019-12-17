#[derive(Default)]
pub struct LxdListener {
    pub on_output: Option<Box<dyn Fn(String) + Send + Sync>>,
}