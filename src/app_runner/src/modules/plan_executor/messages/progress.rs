use actix::{Handler, Message};

use crate::modules::PlanExecutorActor;

#[derive(Message)]
pub struct Progress;

impl Handler<Progress> for PlanExecutorActor {
    type Result = ();

    fn handle(&mut self, _: Progress, _: &mut Self::Context) -> Self::Result {
//        let step = &self.test.steps.get(self.current_step_id);
//
//        match step {
//            Some(step) => {
//                // @todo
//
//                self.current_step_id += 1;
//            }
//
//            None => {}
//        }
    }
}