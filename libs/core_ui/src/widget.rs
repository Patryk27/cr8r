use std::fmt::Display;

pub trait Widget {
    fn print(&self);
    fn println(&self);

    fn eprint(&self);
    fn eprintln(&self);
}

impl<T> Widget for T where T: Display {
    fn print(&self) {
        print!("{}", self);
    }

    fn println(&self) {
        println!("{}", self);
    }

    fn eprint(&self) {
        eprint!("{}", self);
    }

    fn eprintln(&self) {
        eprintln!("{}", self);
    }
}