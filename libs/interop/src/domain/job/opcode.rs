use std::convert::TryFrom;

use crate::convert;
use crate::domain::{DAttachmentId, DomainError, DomainResult};
use crate::proto::core::PJobOpcode;

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
        version: String,
    },

    OverridePackage {
        project: String,
        name: String,
        version: String,
    },

    PatchPackage {
        project: String,
        name: String,
        attachment_id: DAttachmentId,
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

    pub fn override_toolchain(project: impl Into<String>, version: impl Into<String>) -> Self {
        DJobOpcode::OverrideToolchain {
            project: project.into(),
            version: version.into(),
        }
    }

    pub fn override_package(
        project: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        DJobOpcode::OverridePackage {
            project: project.into(),
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn patch_package(
        project: impl Into<String>,
        name: impl Into<String>,
        attachment_id: impl Into<DAttachmentId>,
    ) -> Self {
        DJobOpcode::PatchPackage {
            project: project.into(),
            name: name.into(),
            attachment_id: attachment_id.into(),
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

            Ty::OverrideToolchain(POverrideToolchain { project, version }) => {
                DJobOpcode::OverrideToolchain { project, version }
            }

            Ty::OverridePackage(POverridePackage { project, name, version }) => {
                DJobOpcode::OverridePackage { project, name, version }
            }

            Ty::PatchPackage(PPatchPackage { project, name, attachment_id }) => {
                DJobOpcode::PatchPackage {
                    project,
                    name,
                    attachment_id: convert!(attachment_id as _),
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

            DJobOpcode::OverrideToolchain { project, version } => {
                Ty::OverrideToolchain(POverrideToolchain { project, version })
            }

            DJobOpcode::OverridePackage { project, name, version } => {
                Ty::OverridePackage(POverridePackage { project, name, version })
            }

            DJobOpcode::PatchPackage { project, name, attachment_id } => {
                Ty::PatchPackage(PPatchPackage {
                    project,
                    name,
                    attachment_id: convert!(attachment_id as _),
                })
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}