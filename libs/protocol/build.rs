use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .out_dir("protos/build")
        .compile(&["protos/src/controller.proto"], &["protos/src"])?;

    Ok(())
}