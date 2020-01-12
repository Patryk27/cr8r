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
        tc_version: String,
    },

    OverridePackage {
        project: String,
        pkg_name: String,
        pkg_version: String,
    },

    PatchPackage {
        project: String,
        pkg_name: String,
        pkg_attachment_id: DAttachmentId,
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

    pub fn override_toolchain(project: impl Into<String>, tc_version: impl Into<String>) -> Self {
        DJobOpcode::OverrideToolchain {
            project: project.into(),
            tc_version: tc_version.into(),
        }
    }

    pub fn override_package(
        project: impl Into<String>,
        pkg_name: impl Into<String>,
        pkg_version: impl Into<String>,
    ) -> Self {
        DJobOpcode::OverridePackage {
            project: project.into(),
            pkg_name: pkg_name.into(),
            pkg_version: pkg_version.into(),
        }
    }

    pub fn patch_package(
        project: impl Into<String>,
        pkg_name: impl Into<String>,
        pkg_attachment_id: impl Into<DAttachmentId>,
    ) -> Self {
        DJobOpcode::PatchPackage {
            project: project.into(),
            pkg_name: pkg_name.into(),
            pkg_attachment_id: pkg_attachment_id.into(),
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

            Ty::OverrideToolchain(POverrideToolchain { project, tc_version }) => {
                DJobOpcode::OverrideToolchain { project, tc_version }
            }

            Ty::OverridePackage(POverridePackage { project, pkg_name, pkg_version }) => {
                DJobOpcode::OverridePackage { project, pkg_name, pkg_version }
            }

            Ty::PatchPackage(PPatchPackage { project, pkg_name, pkg_attachment_id }) => {
                DJobOpcode::PatchPackage {
                    project,
                    pkg_name,
                    pkg_attachment_id: convert!(pkg_attachment_id as _),
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

            DJobOpcode::OverrideToolchain { project, tc_version } => {
                Ty::OverrideToolchain(POverrideToolchain { project, tc_version })
            }

            DJobOpcode::OverridePackage { project, pkg_name, pkg_version } => {
                Ty::OverridePackage(POverridePackage { project, pkg_name, pkg_version })
            }

            DJobOpcode::PatchPackage { project, pkg_name, pkg_attachment_id } => {
                Ty::PatchPackage(PPatchPackage {
                    project,
                    pkg_name,
                    pkg_attachment_id: convert!(pkg_attachment_id as _),
                })
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}