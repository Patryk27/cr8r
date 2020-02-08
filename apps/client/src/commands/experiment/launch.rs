use anyhow::*;
use colored::Colorize;
use futures::future::{BoxFuture, FutureExt};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::stream::StreamExt;
use tokio::task;

use lib_core_channel::URx;

use crate::modules::app::AppContext;
use crate::modules::attachment::AttachmentUploaderProgress;
use crate::modules::definition::DefinitionArg;
use crate::modules::experiment::{ExperimentCreator, ExperimentCreatorProgress};

pub async fn launch(ctxt: &mut AppContext, watch: bool, definition: DefinitionArg) -> Result<()> {
    let (creator, mut rx) = ExperimentCreator::new(ctxt);

    task::spawn(async move {
        while let Some(evt) = rx.next().await {
            print(evt, &mut rx)
                .await;
        }
    });

    let id = creator
        .create(definition)
        .await?;

    unimplemented!()
}

fn print(evt: ExperimentCreatorProgress, rx: &mut URx<ExperimentCreatorProgress>) -> BoxFuture<()> {
    use ExperimentCreatorProgress::*;

    async move {
        match evt {
            ValidatingDependencies => {
                println!("{}", "Validating dependencies".green());

                while let Some(evt) = rx.next().await {
                    match evt {
                        ValidatingDependency { name } => {
                            println!("  {} {}", "Validating".cyan(), name);
                        }

                        evt => {
                            print(evt, rx).await;
                            break;
                        }
                    }
                }

                println!();
            }

            UploadingDependencies => {
                println!("{}", "Uploading dependencies".green());

                while let Some(evt) = rx.next().await {
                    match evt {
                        UploadingDependency { name, mut progress } => {
                            let mut upload_pb = None;

                            while let Some(evt) = progress.next().await {
                                match evt {
                                    AttachmentUploaderProgress::CompressingAttachment => {
                                        println!("  {} {}", "Compressing".cyan(), name);
                                    }

                                    AttachmentUploaderProgress::AttachmentCompressed { total_bytes } => {
                                        println!("  {} {}", "  Uploading".cyan(), name);

                                        upload_pb = Some(ProgressBar::new(total_bytes));

                                        let upload_st = ProgressStyle::default_bar()
                                            .template("    {bar:40.blue} {pos:>7}/{len:7} {msg}")
                                            .progress_chars("##-");

                                        upload_pb
                                            .as_ref()
                                            .unwrap()
                                            .set_style(upload_st);
                                    }

                                    AttachmentUploaderProgress::UploadingAttachment { sent_bytes } => {
                                        upload_pb
                                            .as_ref()
                                            .unwrap()
                                            .set_position(sent_bytes);
                                    }

                                    AttachmentUploaderProgress::AttachmentUploaded => {
                                        upload_pb
                                            .as_ref()
                                            .unwrap()
                                            .finish_and_clear();

                                        break;
                                    }
                                }
                            }
                        }

                        _ => {
                            print(evt, rx).await;
                            break;
                        }
                    }
                }

                println!();
            }

            CreatingExperiment => {
                println!("{}", "Creating experiment".green());
            }

            ExperimentCreated => {
                println!("yayayay - finished!");
            }

            _ => unreachable!(),
        }
    }.boxed()
}