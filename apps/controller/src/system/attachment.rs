use tokio::{sync::mpsc, task};

use lib_core_actor::*;
use lib_core_channel::UTx;
use lib_interop::domain::{DAttachmentId, DAttachmentName};
use lib_interop::proto::core::PAttachmentSize;

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Attachment {
    tx: UTx<AttachmentMsg>,
}

impl Attachment {
    pub fn new(id: DAttachmentId, name: DAttachmentName, size: PAttachmentSize) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(AttachmentActor {
            id,
            name,
            size,
        }.start(rx));

        Self { tx }
    }

    pub async fn get_name(&self) -> DAttachmentName {
        ask!(self.tx, AttachmentMsg::GetName)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_name {
        use super::*;

        #[tokio::test]
        async fn returns_attachment_name() {
            let name = DAttachmentName::from("winrar.rar");

            let attachment = Attachment::new(
                123.into(),
                name.clone(),
                4096,
            );

            assert_eq!(
                name,
                attachment.get_name().await,
            );
        }
    }
}