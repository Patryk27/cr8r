use anyhow::*;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::stream::StreamExt;
use tokio::task::spawn;

use lib_core_channel::URx;
use lib_core_ui::*;

use crate::modules::app::AppContext;
use crate::modules::attachment::AttachmentUploaderProgress;
use crate::modules::definition::DefinitionArg;
use crate::modules::experiment::{ExperimentCreator, ExperimentCreatorProgress};

pub async fn launch(ctxt: &mut AppContext, watch: bool, definition: DefinitionArg) -> Result<()> {
    let (creator, mut rx) = ExperimentCreator::new(
        ctxt.conn().await?,
    );

    let ui = spawn(async move {
        while let Some(evt) = rx.next().await {
            print(watch, evt, &mut rx)
                .await;
        }
    });

    let id = creator
        .create(definition)
        .await;

    ui.await?;

    let id = id?;

    if watch {
        super::watch::watch(ctxt, id.into())
            .await
    } else {
        Ok(())
    }
}

async fn print(watch: bool, evt: ExperimentCreatorProgress, rx: &mut URx<ExperimentCreatorProgress>) {
    use ExperimentCreatorProgress::*;

    match evt {
        ValidatingDependencies => {
            print_validating_dependencies(rx)
                .await;
        }

        UploadingDependencies => {
            print_uploading_dependencies(rx)
                .await;
        }

        CreatingExperiment => {
            print_creating_experiment(watch, rx)
                .await;
        }

        _ => unreachable!(),
    }
}

async fn print_validating_dependencies(rx: &mut URx<ExperimentCreatorProgress>) {
    use ExperimentCreatorProgress::*;

    println!("{}", "Validating dependencies".green());

    while let Some(evt) = rx.next().await {
        match evt {
            ValidatingDependency { name } => {
                println!("  {} {}", "Validating".cyan(), name);
            }

            DependenciesValidated => {
                break;
            }

            _ => unreachable!(),
        }
    }

    println!();
}

async fn print_uploading_dependencies(rx: &mut URx<ExperimentCreatorProgress>) {
    use ExperimentCreatorProgress::*;

    println!("{}", "Uploading dependencies".green());

    while let Some(evt) = rx.next().await {
        match evt {
            UploadingDependency { name, progress } => {
                print_uploading_dependency(name, progress)
                    .await;
            }

            DependenciesUploaded => {
                break;
            }

            _ => unreachable!(),
        }
    }

    println!()
}

async fn print_uploading_dependency(name: String, mut rx: URx<AttachmentUploaderProgress>) {
    use AttachmentUploaderProgress::*;

    let mut progress = None;

    while let Some(evt) = rx.next().await {
        match evt {
            CompressingAttachment => {
                println!("  {} {}", "Compressing".cyan(), name);
            }

            AttachmentCompressed { total_bytes } => {
                println!("  {} {}", "  Uploading".cyan(), name);

                progress = Some(ProgressBar::new(total_bytes));

                let style = ProgressStyle::default_bar()
                    .template("    {bar:40.blue} {pos:>7}/{len:7} {msg}")
                    .progress_chars("##-");

                progress
                    .as_ref()
                    .unwrap()
                    .set_style(style);
            }

            UploadingAttachment { sent_bytes } => {
                progress
                    .as_ref()
                    .unwrap()
                    .set_position(sent_bytes);
            }

            AttachmentUploaded => {
                progress
                    .as_ref()
                    .unwrap()
                    .finish_and_clear();

                break;
            }
        }
    }
}

// @todo add spinner
async fn print_creating_experiment(watch: bool, rx: &mut URx<ExperimentCreatorProgress>) {
    use ExperimentCreatorProgress::*;

    println!("{}", "Creating experiment".green());

    while let Some(evt) = rx.next().await {
        match evt {
            ExperimentCreated { id } => {
                println!();

                MessageWidget::success("Success:", [
                    format!("Experiment `{}` has been created.", id.to_string().blue()),
                    "It's now waiting for a runner to pick it up.".to_string(),
                ]).println();

                if !watch {
                    println!("You can see status of your experiment using:");
                    println!("$ {}", format!("cr8r experiment show {}", id).blue());
                    println!();
                    println!("Or, if you prefer a real-time view:");
                    println!("$ {}", format!("cr8r experiment watch {}", id).blue());
                }
            }

            _ => unreachable!(),
        }
    }
}