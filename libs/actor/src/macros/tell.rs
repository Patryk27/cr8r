#[macro_export]
macro_rules! tell {
    ($tx:expr, $msg:expr) => {{
        let _ = $tx.send($msg);
    }};
}