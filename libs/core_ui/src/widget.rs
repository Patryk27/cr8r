use std::fmt;

pub trait Widget {
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    fn writeln(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn print(&self);
    fn println(&self);

    fn eprint(&self);
    fn eprintln(&self);
}

impl<T> Widget for T where T: fmt::Display {
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }

    fn writeln(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self)
    }

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