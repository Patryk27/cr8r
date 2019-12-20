use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .format(true)
        .out_dir("protobuf/build")
        .compile(&[
            "protobuf/src/for_client.proto",
            "protobuf/src/for_runner.proto",
        ], &[
            "protobuf/src",
        ])?;

    Ok(())
}