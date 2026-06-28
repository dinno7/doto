use crate::{adapters::cli::use_cases::TodoUCs, shared::utils::is_user_confirmed};
use clap::Args;

#[derive(Debug, Args)]
pub struct RmArgs {
    id: Option<u64>,

    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    status: Option<String>,

    #[arg(short, long)]
    done: bool,
}

pub async fn handle(uc: &TodoUCs, args: RmArgs) {
    if let Some(id) = args.id {
        match uc.delete_todo_by_id.execute(id).await {
            Ok(true) => println!("Todo deleted"),
            Ok(false) => eprintln!("Todo not found"),
            Err(e) => eprintln!("Failed to delete todo: {e}"),
        }
        return;
    }

    if args.all && is_user_confirmed("This action delete all your todos PERMANENTLY!") {
        match uc.delete_all_todos_by_status.execute(None).await {
            Ok(affected) => println!("{affected} todos deleted"),
            Err(e) => eprintln!("Failed to delete todo: {e}"),
        };
        return;
    }

    if args.done && is_user_confirmed("This action delete all your \"Done\" todos PERMANENTLY!") {
        match uc.delete_all_todos_by_status.execute(Some("done")).await {
            Ok(affected) => println!("{affected} todos with Done status deleted"),
            Err(e) => eprintln!("Failed to delete todo: {e}"),
        };
        return;
    }

    if let Some(status) = args.status
        && is_user_confirmed(&format!(
            "This action delete all your \"{status}\" todos PERMANENTLY!"
        ))
    {
        match uc.delete_all_todos_by_status.execute(Some(&status)).await {
            Ok(affected) => println!("{affected} todos with {status} status deleted"),
            Err(e) => eprintln!("Failed to delete todo: {e}"),
        };
        return;
    }
    println!("See --help")
}
