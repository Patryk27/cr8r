use bastion::prelude::*;

#[derive(Debug)]
enum Command {
    Hello,
}

pub struct System {
    conn: ChildRef,
}

impl System {
    pub fn new(conn: ChildRef) -> Self {
        Self { conn }
    }

    pub async fn start(ctx: BastionContext) -> Result<(), ()> {
        loop {
            let mut packet = ctx.recv().await?;

            let tx = packet.take_sender().unwrap();
            let cmd = packet.downcast().unwrap(): Command;

            match cmd {
                Command::Hello => {
                    tx.send("Hi".to_string());
                }
            }
        }
    }

    pub async fn hello(&self) -> String {
        self.conn
            .ask(Command::Hello)
            .unwrap()
            .await
            .unwrap()
            .downcast()
            .unwrap()
    }
}