pub mod client {
    include!("../protobuf/build/cr8r.client.rs");
}

pub mod core {
    include!("../protobuf/build/cr8r.core.rs");

    // Experiment-oriented types
    pub type ExperimentId = String;

    // Runner-oriented types
    pub type RunnerId = String;
    pub type RunnerName = String;
    pub type RunnerSecret = String;
}

pub mod runner {
    include!("../protobuf/build/cr8r.runner.rs");
}