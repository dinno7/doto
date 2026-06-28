use clap::Subcommand;

use crate::adapters::cli::use_cases::CliUseCases;

pub mod add;
pub mod ls;
pub mod rm;

#[derive(Debug, Subcommand)]
pub enum TodoCommands {
    Add(add::AddArgs),
    Ls(ls::LsArgs),
    Rm(rm::RmArgs),
}

pub async fn dispatch(uc: &CliUseCases, cmd: TodoCommands) {
    match cmd {
        TodoCommands::Add(args) => add::handle(&uc.todos, args).await,
        TodoCommands::Ls(args) => ls::handle(&uc.todos, args).await,
        TodoCommands::Rm(args) => rm::handle(&uc.todos, args).await,
    }
}
