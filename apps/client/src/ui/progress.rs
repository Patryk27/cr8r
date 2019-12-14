#[macro_export]
macro_rules! spinner {
    ($expr:expr) => {{
        use indicatif::ProgressBar;

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(100);

        let expr = $expr;

        pb.finish_and_clear();

        expr
    }}
}