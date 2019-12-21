pub struct Command {
    cmd: String,
}

impl Command {
    pub fn new(cmd: String) -> Self {
        Self { cmd }
    }
}