use bastion::Bastion;

pub use self::system::*;

mod system;

pub fn start() -> System {
    Bastion::init();

    let children = Bastion::children(|children| {
        children.with_exec(System::start)
    }).unwrap();

    Bastion::start();

    System::new(
        children.elems()[0].clone()
    )
}