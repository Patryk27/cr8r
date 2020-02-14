use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .format(true)
        .out_dir("protobuf/.artifacts")
        .compile(
            &["protobuf/controller.proto", "protobuf/core.proto"],
            &["protobuf"],
        )?;

    Ok(())
}