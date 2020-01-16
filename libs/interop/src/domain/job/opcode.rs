use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::proto::core::PJobOpcode;

pub use self::override_dependency_action::*;

mod override_dependency_action;

#[derive(Clone, Debug)]
pub enum DJobOpcode {
    LogSystemMsg {
        msg: String,
    },

    LogCustomMsg {
        msg: String,
    },

    InvokeCmd {
        cmd: String,
    },

    OverrideToolchain {
        project: String,
        toolchain: String,
    },

    OverrideDependency {
        project: String,
        registry: String,
        name: String,
        action: DOverrideDependencyAction,
    },
}

impl DJobOpcode {
    pub fn log_system_msg(msg: impl Into<String>) -> Self {
        DJobOpcode::LogSystemMsg {
            msg: msg.into(),
        }
    }

    pub fn log_custom_msg(msg: impl Into<String>) -> Self {
        DJobOpcode::LogCustomMsg {
            msg: msg.into(),
        }
    }

    pub fn invoke_cmd(cmd: impl Into<String>) -> Self {
        DJobOpcode::InvokeCmd {
            cmd: cmd.into(),
        }
    }

    pub fn override_toolchain(project: impl Into<String>, toolchain: impl Into<String>) -> Self {
        DJobOpcode::OverrideToolchain {
            project: project.into(),
            toolchain: toolchain.into(),
        }
    }

    pub fn override_dependency(
        project: impl Into<String>,
        registry: impl Into<String>,
        name: impl Into<String>,
        action: DOverrideDependencyAction,
    ) -> Self {
        DJobOpcode::OverrideDependency {
            project: project.into(),
            registry: registry.into(),
            name: name.into(),
            action,
        }
    }
}

impl TryFrom<PJobOpcode> for DJobOpcode {
    type Error = DomainError;

    fn try_from(PJobOpcode { ty }: PJobOpcode) -> DomainResult<Self> {
        use crate::proto::core::p_job_opcode::*;

        Ok(match convert!(ty?) {
            Ty::LogSystemMsg(PLogSystemMsg { msg }) => {
                DJobOpcode::LogSystemMsg { msg }
            }

            Ty::LogCustomMsg(PLogCustomMsg { msg }) => {
                DJobOpcode::LogCustomMsg { msg }
            }

            Ty::InvokeCmd(PInvokeCmd { cmd }) => {
                DJobOpcode::InvokeCmd { cmd }
            }

            Ty::OverrideToolchain(POverrideToolchain { project, toolchain }) => {
                DJobOpcode::OverrideToolchain { project, toolchain }
            }

            Ty::OverrideDependency(POverrideDependency { project, registry, name, action }) => {
                DJobOpcode::OverrideDependency {
                    project,
                    registry,
                    name,
                    action: convert!(action? as _?),
                }
            }
        })
    }
}

impl Into<PJobOpcode> for DJobOpcode {
    fn into(self) -> PJobOpcode {
        use crate::proto::core::p_job_opcode::*;

        let ty = match self {
            DJobOpcode::LogSystemMsg { msg } => {
                Ty::LogSystemMsg(PLogSystemMsg { msg })
            }

            DJobOpcode::LogCustomMsg { msg } => {
                Ty::LogCustomMsg(PLogCustomMsg { msg })
            }

            DJobOpcode::InvokeCmd { cmd } => {
                Ty::InvokeCmd(PInvokeCmd { cmd })
            }

            DJobOpcode::OverrideToolchain { project, toolchain } => {
                Ty::OverrideToolchain(POverrideToolchain { project, toolchain })
            }

            DJobOpcode::OverrideDependency { project, registry, name, action } => {
                Ty::OverrideDependency(POverrideDependency {
                    project,
                    registry,
                    name,
                    action: Some(convert!(action as _)),
                })
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}