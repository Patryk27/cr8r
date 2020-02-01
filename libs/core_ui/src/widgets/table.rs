#[macro_export]
macro_rules! table {
    (
        titles: [ $($title:expr),+ ],
    ) => {{
        use prettytable::*;

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!( $( $title, )+ ));

        table
    }};
}
