pub mod core {
    include!("../protobuf/.artifacts/cr8r.core.rs");

    // Experiment-oriented types
    pub type PExperimentId = String;

    // Runner-oriented types
    pub type PRunnerId = String;
    pub type PRunnerName = String;
    pub type PRunnerSecret = String;
}

pub mod controller {
    include!("../protobuf/.artifacts/cr8r.controller.rs");
}