use lib_lxd::LxdDeviceDef;

#[derive(Debug)]
pub enum SandboxMount {
    File {
        host: String,
        sandbox: String,
    },
}

impl SandboxMount {
    crate fn into_device_def(self) -> LxdDeviceDef {
        match self {
            SandboxMount::File { host, sandbox } => {
                LxdDeviceDef::Disk {
                    source: host,
                    path: sandbox,
                }
            }
        }
    }
}