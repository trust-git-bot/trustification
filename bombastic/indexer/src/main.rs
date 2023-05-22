use std::process::{ExitCode, Termination};

use bombastic_indexer::Run;
use clap::Parser;

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Run(Run),
}

#[derive(clap::Parser, Debug)]
#[command(
    author,
    version = env!("CARGO_PKG_VERSION"),
    about = "Bombastic Indexer",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    async fn run(self) -> ExitCode {
        match self.run_command().await {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        }
    }

    async fn run_command(self) -> anyhow::Result<ExitCode> {
        match self.command {
            Command::Run(run) => {
                run.run().await?;
            }
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[tokio::main]
async fn main() -> impl Termination {
    tracing_subscriber::fmt::init();
    Cli::parse().run().await
}