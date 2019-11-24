use actix::{Actor, Handler, Message};

use lib_protocol::ExecutionStep;

use crate::modules::TaskExecutorActor;

#[derive(Message)]
pub struct ExecuteStep {
    pub step: ExecutionStep,
}

impl Handler<ExecuteStep> for TaskExecutorActor {
    type Result = ();

    fn handle(&mut self, msg: ExecuteStep, _: &mut Self::Context) -> Self::Result {
        debug!("Running step: {:?}", step);

        match step {
            ExecutionStep::Log { message: _ } => {
                // @todo
            }

            ExecutionStep::RunCommand { command } => {
                let out = Command::new("/snap/bin/lxc")
                    .args(&["exec", &self.container_name(), "--", "bash", "-c", command])
                    .output()
                    .unwrap();

                if !out.status.success() {
                    panic!("ayy ayy");
                }
            }

            ExecutionStep::RunCommands { commands } => {
                let command = commands.join(" && ");

                let out = Command::new("/snap/bin/lxc")
                    .args(&["exec", &self.container_name(), "--", "bash", "-c", &command])
                    .output()
                    .unwrap();

                if !out.status.success() {
                    panic!("ayy ayy");
                }
            }

            ExecutionStep::Start => {
                panic!("starty starty");
            }
        }
    }
}