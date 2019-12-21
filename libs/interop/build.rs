use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .format(true)
        .out_dir("protobuf/.artifacts")
        .compile(
            &["protobuf/for_client.proto", "protobuf/for_runner.proto"],
            &["protobuf"],
        )?;

    Ok(())
}