pub mod core {
    include!("../protobuf/.artifacts/cr8r.core.rs");

    // Experiment-oriented types
    pub type PExperimentId = String;

    // Runner-oriented types
    pub type PRunnerId = String;
    pub type PRunnerName = String;
    pub type PRunnerSecret = String;
}

pub mod for_client {
    include!("../protobuf/.artifacts/cr8r.for_client.rs");
}

pub mod for_runner {
    include!("../protobuf/.artifacts/cr8r.for_runner.rs");
}