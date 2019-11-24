use std::process::Command;

use actix::{Actor, Addr, AsyncContext, Context};
use log::*;

use lib_protocol::{ExecutionPlan, ExecutionStep, ExperimentId};

use crate::modules::TaskExecutorActor;

mod messages;

pub struct PlanExecutorActor {
    plan: ExecutionPlan,
    step_executor: Addr<TaskExecutorActor>,
    current_step: usize,
}

impl PlanExecutorActor {
    pub fn new(plan: ExecutionPlan, step_executor: Addr<TaskExecutorActor>) -> Self {
        Self {
            plan,
            step_executor,
            current_step: 0,
        }
    }

    fn container_name(&self) -> String {
        format!("cr8r-{}", self.id)
    }
}

impl Actor for PlanExecutorActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("Actor started.");
        debug!("-> plan: {:?}", self.plan);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        debug!("Actor stopped.");
    }
}

