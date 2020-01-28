use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DomainError, DomainResult};
use crate::domain::definition::definition_inner::{DDependencyDef, DToolchainDef};
use crate::proto::core::PJobOpcode;

#[derive(Clone, Debug)]
pub enum DJobOpcode {
    LogSystemMsg {
        msg: String,
    },

    LogCustomMsg {
        msg: String,
    },

    Execute {
        cmd: String,
    },

    AlterToolchain {
        project: String,
        toolchain: DToolchainDef,
    },

    AlterDependency {
        project: String,
        dependency: DDependencyDef,
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

    pub fn execute(cmd: impl Into<String>) -> Self {
        DJobOpcode::Execute {
            cmd: cmd.into(),
        }
    }

    pub fn alter_toolchain(project: impl Into<String>, toolchain: DToolchainDef) -> Self {
        DJobOpcode::AlterToolchain {
            project: project.into(),
            toolchain,
        }
    }

    pub fn patch_dependency(project: impl Into<String>, dependency: DDependencyDef) -> Self {
        DJobOpcode::AlterDependency {
            project: project.into(),
            dependency,
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

            Ty::Execute(PExecute { cmd }) => {
                DJobOpcode::Execute { cmd }
            }

            Ty::AlterToolchain(PAlterToolchain { project, toolchain }) => {
                DJobOpcode::AlterToolchain {
                    project,
                    toolchain: convert!(toolchain? as _?),
                }
            }

            Ty::AlterDependency(PAlterDependency { project, dependency }) => {
                DJobOpcode::AlterDependency {
                    project,
                    dependency: convert!(dependency? as _?),
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

            DJobOpcode::Execute { cmd } => {
                Ty::Execute(PExecute { cmd })
            }

            DJobOpcode::AlterToolchain { project, toolchain } => {
                Ty::AlterToolchain(PAlterToolchain {
                    project,
                    toolchain: Some(convert!(toolchain as _)),
                })
            }

            DJobOpcode::AlterDependency { project, dependency } => {
                Ty::AlterDependency(PAlterDependency {
                    project,
                    dependency: Some(convert!(dependency as _)),
                })
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}