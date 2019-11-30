use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .out_dir("protobuf/build")
        .compile(&[
            "protobuf/src/client.proto",
            "protobuf/src/runner.proto",
        ], &[
            "protobuf/src",
        ])?;

    Ok(())
}