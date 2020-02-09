pub mod core {
    include!("../protobuf/.artifacts/cr8r.core.rs");

    // Attachment-oriented types
    pub type PAttachmentId = u32;
    pub type PAttachmentName = String;
    pub type PAttachmentSize = u64;

    // Experiment-oriented types
    pub type PExperimentId = u32;

    // Runner-oriented types
    pub type PRunnerId = u32;
    pub type PRunnerName = String;
    pub type PRunnerSecret = String;
}

pub mod controller {
    include!("../protobuf/.artifacts/cr8r.controller.rs");
}