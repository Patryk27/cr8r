#[macro_export]
macro_rules! ask {
    (
        $tx:expr,

        // Sadly, it seems we can't use `:path` to parse `Foo::Bar` as an enum variant, so we gotta resort to parsing it
        // manually in parts
        $($msg_a:ident)+
        $(::$msg_b:ident)*

        $( { $($msg_arg:ident),+ })?
    ) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();

        let msg = $($msg_a)+$(::$msg_b)* {
            $( $($msg_arg,)+ )?
            tx
        };

        if $tx.send(msg).is_ok() {
            if let Ok(rx) = rx.await {
                rx
            } else {
                panic!("Connection to actor lost (could not await response) - did it die prematurely?"); // @todo
            }
        } else {
            panic!("Connection to actor lost (could not send message) - did it die prematurely?"); // @todo
        }
    }};
}
