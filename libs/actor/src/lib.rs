#[macro_export]
macro_rules! ask {
    (
        $tx:expr,

        // Since our messages are enums, we can't just write `$msg:ty` and call it a day, because rustc won't properly
        // parse the entire macro
        $($msg_a:ident)+
        $(::$msg_b:ident)*

        $( { $($msg_arg:ident),+ })?
    ) => {{
        let (tx, rx) = futures_channel::oneshot::channel();

        let msg = $($msg_a)+$(::$msg_b)* {
            $( $($msg_arg,)+ )?
            tx
        };

        if $tx.unbounded_send(msg).is_ok() {
            if let Ok(rx) = rx.await {
                rx
            } else {
                panic!("Failed to await actor's response - did it die prematurely?"); // @todo
            }
        } else {
            panic!("Failed to send message to the actor - did it die prematurely?"); // @todo
        }
    }};
}

#[macro_export]
macro_rules! tell {
    ($tx:expr, $msg:expr) => {{
        if !$tx.unbounded_send($msg).is_ok() {
            panic!("Failed to send message to the actor - did it die prematurely?"); // @todo
        }
    }};
}