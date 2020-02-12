use std::fmt;

use colored::Colorize;

pub use self::{
    body::*,
    r#type::*,
};

mod body;
mod r#type;

pub struct MessageWidget {
    ty: MessageType,
    header: String,
    body: String,
    invert_colors: bool,
}

impl MessageWidget {
    pub fn new(ty: MessageType, header: String, body: impl MessageBody, invert_colors: bool) -> Self {
        Self {
            ty,
            header,
            body: body.body(),
            invert_colors,
        }
    }

    fn colored(&self, text: &str, use_color: bool) -> String {
        use MessageType::*;

        if !use_color {
            return text.to_string();
        }

        match self.ty {
            Info => text.blue(),
            Warn => text.yellow(),
            Error => text.red(),
            Success => text.green(),
        }.to_string()
    }
}

macro_rules! constructors {
    ( $($ty:path => $fn:ident $fn_inv:ident,)* ) => {
        impl MessageWidget {
        $(
            pub fn $fn(header: impl Into<String>, body: impl MessageBody) -> Self {
                Self::new($ty, header.into(), body, false)
            }

            pub fn $fn_inv(header: impl Into<String>, body: impl MessageBody) -> Self {
                Self::new($ty, header.into(), body, true)
            }
        )*
        }
    }
}

constructors! {
    MessageType::Info => info info_inv,
    MessageType::Warn => warn warn_inv,
    MessageType::Error => error error_inv,
    MessageType::Success => success success_inv,
}

impl fmt::Display for MessageWidget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_header = !self.invert_colors;
        let color_body = self.invert_colors;

        writeln!(
            f,
            "{}",
            self.colored(&self.header, color_header),
        )?;

        for line in self.body.lines() {
            writeln!(
                f,
                "  {}",
                self.colored(line, color_body)
            )?;
        }

        Ok(())
    }
}
