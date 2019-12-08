#[derive(Debug)]
pub enum LxdDeviceDef {
    Disk {
        source: String,
        path: String,
    }
}

impl LxdDeviceDef {
    pub fn into_args(self) -> Vec<String> {
        match self {
            LxdDeviceDef::Disk { source, path } => {
                vec![
                    "disk".to_string(),
                    format!("source={}", source),
                    format!("path={}", path),
                ]
            }
        }
    }
}