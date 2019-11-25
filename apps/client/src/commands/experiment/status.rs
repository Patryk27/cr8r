use crate::{Result, System};

pub fn run(system: System, id: String) -> Result<()> {
    let experiment = system
        .connector()
        .experiment(id)?;

    println!("{:#?}", experiment);

    Ok(())
}