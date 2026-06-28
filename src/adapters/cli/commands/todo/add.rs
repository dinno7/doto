use crate::{
    adapters::cli::use_cases::TodoUCs, application::usecases::todos::create_todo::CreateTodoInput,
};

use clap::Args;

#[derive(Debug, Args)]
pub struct AddArgs {
    pub title: String,

    #[arg(short, long)]
    pub priority: Option<String>,

    #[arg(short, long)]
    pub description: Option<String>,

    #[arg(short, long)]
    pub tags: Option<Vec<String>>,
}

pub async fn handle(uc: &TodoUCs, args: AddArgs) {
    let resolved_priority = args.priority.as_ref().map(|p| resolve_priority(p));
    let input = CreateTodoInput::new(
        &args.title,
        args.description,
        resolved_priority,
        args.tags.unwrap_or_default(),
        None,
    );
    match uc.create_todo.execute(input).await {
        Ok(()) => println!("Todo created"),
        Err(e) => eprintln!("Something went wrong you can use --help: {e}"),
    }
}

pub fn resolve_priority(raw: &str) -> String {
    if let Ok(n) = raw.parse::<u32>() {
        return match n {
            0 | 1 => "critical".to_string(),
            2 => "high".to_string(),
            3 => "medium".to_string(),
            _ => "low".to_string(),
        };
    }
    if raw.len() == 1 {
        return match raw.to_lowercase().as_str() {
            "c" => "critical",
            "h" => "high",
            "m" => "medium",
            _ => "low",
        }
        .to_string();
    }
    raw.to_string()
}
