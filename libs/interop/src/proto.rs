pub mod models {
    include!("../protobuf/.artifacts/cr8r.models.rs");

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

pub mod services {
    include!("../protobuf/.artifacts/cr8r.services.rs");
}