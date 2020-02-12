use itertools::Itertools;

pub trait MessageBody {
    fn body(&self) -> String;
}

impl<'a> MessageBody for &'a str {
    fn body(&self) -> String {
        self.to_owned()
            .to_owned()
    }
}

impl MessageBody for String {
    fn body(&self) -> String {
        self.to_owned()
    }
}

impl<T: MessageBody, const N: usize> MessageBody for [T; N] {
    fn body(&self) -> String {
        self.iter()
            .map(|body| body.body())
            .join("\n")
    }
}