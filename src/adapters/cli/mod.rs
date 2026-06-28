use std::path::PathBuf;

use crate::adapters::cli::commands::todo::TodoCommands;
use clap::Parser;
use clap::Subcommand;
use use_cases::CliUseCases;

pub mod commands;
pub mod use_cases;

pub struct CliAdapter {
    use_cases: CliUseCases,
}

impl CliAdapter {
    pub fn new(use_cases: CliUseCases) -> Self {
        Self { use_cases }
    }

    pub async fn run(&self) {
        let cli = Cli::parse();

        if let Some(todo_commands) = cli.todo_commands {
            commands::todo::dispatch(&self.use_cases, todo_commands).await;
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub todo_commands: Option<TodoCommands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(subcommand)]
    Todo(TodoCommands),
}
