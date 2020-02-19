#[macro_export]
macro_rules! table {
    (
        format: $format:ident,
        titles: [ $($title:expr),+ ],
    ) => {{
        use prettytable::*;

        let mut table = Table::new();

        table.set_format(*format::consts::$format);
        table.set_titles(row!( $( $title, )+ ));

        table
    }};
}
