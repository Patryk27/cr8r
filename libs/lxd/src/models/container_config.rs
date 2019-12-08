use crate::{LxdContainerName, LxdDeviceDef, LxdDeviceName};

#[derive(Debug)]
pub enum LxdContainerConfig {
    AddDevice {
        name: LxdDeviceName,
        def: LxdDeviceDef,
    },

    Set {
        key: String,
        value: String,
    },
}

impl LxdContainerConfig {
    pub fn into_args(self, container: &LxdContainerName) -> Vec<String> {
        match self {
            LxdContainerConfig::AddDevice { name, def } => {
                let mut args = vec![
                    "device".to_string(),
                    "add".to_string(),
                    container.to_string(),
                    name.to_string(),
                ];

                args.extend(def.into_args());
                args
            }

            LxdContainerConfig::Set { key, value } => {
                vec![
                    "set".to_string(),
                    container.to_string(),
                    key,
                    value,
                ]
            }
        }
    }
}

