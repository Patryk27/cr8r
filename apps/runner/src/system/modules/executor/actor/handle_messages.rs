use log::*;

use lib_core_actor::*;

use super::ExecutorActor;

impl ExecutorActor {
    pub(super) fn handle_messages(&mut self) -> ActorWorkflow {
        while let Ok(msg) = self.mailbox.try_recv() {
            if msg.handle(self).actor_should_stop() {
                trace!("Received termination signal, actor will get stopped");
                return ActorWorkflow::Stop;
            }
        }

        ActorWorkflow::Continue
    }

    pub(super) async fn handle_messages_until_orphaning(mut self) {
        while let Some(msg) = self.mailbox.recv().await {
            if msg.handle(&mut self).actor_should_stop() {
                trace!("Received termination signal, actor will get stopped");
                return;
            }
        }
    }
}