use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use atty::Stream;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use log::{error, info, warn};
use tokio::io::AsyncReadExt;

use crate::{
    config::types::Config,
    pastebins::{PasteBinMeta, PasteBins, pastebin_com::PastebinCom},
};

mod config;
mod pastebins;

#[derive(Parser, Debug)]
#[command(version, long_about = None, name = "kcli")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand_required = true, arg_required_else_help = true)]
    Config {
        #[command(subcommand)]
        command: Option<GenerateCompletionsCommands>,
    },
    Upload {
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    GenerateCompletions {
        #[arg(short, long)]
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
enum GenerateCompletionsCommands {
    Show,
    Edit,
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    colog::init();

    let cli = Cli::parse();

    if let Some(Commands::GenerateCompletions { shell }) = cli.command {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {shell:?}...");
        print_completions(shell, &mut cmd);
        std::process::exit(0);
    }

    let (cfg, cfg_path) = Config::load().await?;

    if let Some(Commands::Config { command }) = cli.command {
        match command {
            Some(GenerateCompletionsCommands::Edit) => {
                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());

                let status = tokio::process::Command::new(editor)
                    .arg(&cfg_path)
                    .status()
                    .await
                    .context("failed to open editor")?;

                if !status.success() {
                    eprintln!("Editor exited with non-zero status");
                }
            }
            Some(GenerateCompletionsCommands::Show) => {
                println!("{:#?}", cfg);
            }
            None => {}
        }
        std::process::exit(0);
    }

    let mut pastebins = PasteBins::new();
    if cfg.pastebin_com.enable {
        match cfg.pastebin_com.key {
            Some(key) => {
                if key.trim().is_empty() {
                    warn!(
                        "failed to register ({} / {}) because key is emmpty",
                        PastebinCom::DISPLAY_NAME,
                        PastebinCom::DOMAIN
                    );
                }
                pastebins.register(Arc::new(PastebinCom::new(&key)));
            }
            None => {
                warn!(
                    "failed to register ({} / {}) because key is not set",
                    PastebinCom::DISPLAY_NAME,
                    PastebinCom::DOMAIN
                );
            }
        }
    }

    if let Some(Commands::Upload { file }) = cli.command {
        let content = if let Some(file) = file {
            tokio::fs::read_to_string(file).await?
        } else if !atty::is(Stream::Stdin) {
            let mut input = String::new();
            tokio::io::stdin().read_to_string(&mut input).await?;
            input
        } else {
            error!("no input provided (file or stdin)");
            std::process::exit(1);
        };

        let service = pastebins
            .get(PastebinCom::ID)
            .expect("pastebin service not registered");

        let url = service.upload(&content).await?;
        info!("{url}");
    }

    Ok(())
}
