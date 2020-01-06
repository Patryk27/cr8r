#[macro_export]
macro_rules! spinner {
    ($expr:expr) => {{
        use anyhow::Result;
        use indicatif::ProgressBar;

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(100);

        let expr = try {
            $expr
        }: Result<_>;

        pb.finish_and_clear();

        expr?
    }}
}