use crate::backend::{Actor, Msg};

impl Msg {
    fn process(self, actor: &mut Actor) {
        match msg {
            Msg::AsModel { tx } => {
                let _ = tx.send(self.process_as_model());
            }

            Msg::Report { runner, report, tx } => {
                let _ = tx.send(self.process_report(runner, report));
            }

            Msg::Start { runner, tx } => {
                let _ = tx.send(self.process_start(runner));
            }

            Msg::Watch { tx } => {
                let _ = tx.send(self.process_watch());
            }
        }
    }
}

fn process_as_model(actor)