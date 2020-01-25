#[derive(Debug)]
pub enum LxdDeviceDef {
    Disk {
        source: String,
        path: String,
    }
}

