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

    OverrideDependency {
        project: String,
        dep_registry: String,
        dep_name: String,
        dep_version: String,
    },

    PatchDependency {
        project: String,
        dep_registry: String,
        dep_name: String,
        dep_source_attachment_id: DAttachmentId,
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

    pub fn override_dependency(
        project: impl Into<String>,
        dep_registry: impl Into<String>,
        dep_name: impl Into<String>,
        dep_version: impl Into<String>,
    ) -> Self {
        DJobOpcode::OverrideDependency {
            project: project.into(),
            dep_registry: dep_registry.into(),
            dep_name: dep_name.into(),
            dep_version: dep_version.into(),
        }
    }

    pub fn patch_dependency(
        project: impl Into<String>,
        dep_registry: impl Into<String>,
        dep_name: impl Into<String>,
        dep_source_attachment_id: impl Into<DAttachmentId>,
    ) -> Self {
        DJobOpcode::PatchDependency {
            project: project.into(),
            dep_registry: dep_registry.into(),
            dep_name: dep_name.into(),
            dep_source_attachment_id: dep_source_attachment_id.into(),
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

            Ty::OverrideDependency(POverrideDependency { project, dep_registry, dep_name, dep_version }) => {
                DJobOpcode::OverrideDependency { project, dep_registry, dep_name, dep_version }
            }

            Ty::PatchDependency(PPatchDependency { project, dep_registry, dep_name, dep_source_attachment_id }) => {
                DJobOpcode::PatchDependency {
                    project,
                    dep_registry,
                    dep_name,
                    dep_source_attachment_id: convert!(dep_source_attachment_id as _),
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

            DJobOpcode::OverrideDependency { project, dep_registry, dep_name, dep_version } => {
                Ty::OverrideDependency(POverrideDependency { project, dep_registry, dep_name, dep_version })
            }

            DJobOpcode::PatchDependency { project, dep_registry, dep_name, dep_source_attachment_id } => {
                Ty::PatchDependency(PPatchDependency {
                    project,
                    dep_registry,
                    dep_name,
                    dep_source_attachment_id: convert!(dep_source_attachment_id as _),
                })
            }
        };

        PJobOpcode { ty: Some(ty) }
    }
}