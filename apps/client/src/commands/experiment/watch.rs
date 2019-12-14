use colored::Colorize;

use lib_protocol::for_client::{PWatchExperimentReply, PWatchExperimentRequest};
use lib_protocol::for_client::p_watch_experiment_reply::Kind;

use crate::{Result, spinner, System};
use crate::ui::reformat_datetime;

pub async fn run(mut system: System, id: String) -> Result<()> {
    println!("Attaching to experiment `{}`", id.blue());

    let mut reply = spinner! {
        system
            .client().await?
            .watch_experiment(PWatchExperimentRequest { id }).await?
            .into_inner()
    };

    println!("Attached, logs follow:");
    println!();

    loop {
        let reply = spinner! {
            reply
                .message()
                .await?
        };

        if let Some(reply) = reply {
            if reply.kind == Kind::StreamClosed as i32 {
                break;
            }

            if let Some(reply) = reply_to_string(reply) {
                println!("{}", reply);
            }
        }
    }

    println!();
    println!("Stream closed");

    Ok(())
}

fn reply_to_string(reply: PWatchExperimentReply) -> Option<String> {
    let kind = Kind::from_i32(reply.kind)
        .unwrap_or(Kind::SystemMessage);

    let msg_datetime = reformat_datetime(&reply.created_at)
        .dimmed();

    let msg_kind = kind_to_string(kind)
        .dimmed();

    let msg_content = match kind {
        Kind::StreamClosed => return None,

        Kind::SystemMessage => reply.message
            .blue()
            .to_string(),

        Kind::UserMessage => reply.message
            .white()
            .to_string(),

        Kind::ProcessOutput => reply.message
            .white()
            .dimmed()
            .to_string(),
    };

    Some(format!("{} {} | {}", msg_datetime, msg_kind, msg_content))
}

fn kind_to_string(kind: Kind) -> &'static str {
    //@formatter:off
    match kind {
        Kind::StreamClosed  => "    ",
        Kind::SystemMessage => "sys ",
        Kind::UserMessage   => "msg ",
        Kind::ProcessOutput => "proc",
    }
    //@formatter:on
}