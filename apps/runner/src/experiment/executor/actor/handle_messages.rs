use log::*;

use lib_core_actor::*;

use super::ExperimentExecutorActor;

impl ExperimentExecutorActor {
    pub(super) fn handle_messages(&mut self) -> ActorWorkflow {
        while let Ok(msg) = self.mailbox.try_recv() {
            if msg.handle(self).actor_should_stop() {
                debug!("Received termination signal, actor will get stopped.");
                return ActorWorkflow::Stop;
            }
        }

        ActorWorkflow::Continue
    }

    pub(super) async fn handle_messages_until_orphaning(mut self) {
        debug!("Actor finished working, entering event loop");

        while let Some(msg) = self.mailbox.recv().await {
            if msg.handle(&mut self).actor_should_stop() {
                debug!("Received termination signal, actor will get stopped.");
                return;
            }
        }

        debug!("Actor has halted");
    }
}