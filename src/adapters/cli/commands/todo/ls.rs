use std::io::stdout;

use crate::{
    adapters::cli::use_cases::TodoUCs,
    application::usecases::todos::get_all_todos::GetAllTodosInput,
    domain::{
        entities::todo::{Priority, Todo},
        vo::todo_status::TodoStatus,
    },
    shared::{tabwriter::TabWriter, utils::time_ago},
};

use clap::Args;
use console::style;

#[derive(Debug, Args)]
pub struct LsArgs {
    #[arg(short, long)]
    all: bool,
}

pub async fn handle(uc: &TodoUCs, args: LsArgs) {
    let input = GetAllTodosInput {
        with_tags: args.all,
    };

    let todos = uc.get_all_todos.execute(input).await.unwrap();
    if todos.is_empty() {
        println!("No any todo found!");
        return;
    }
    print_todo_table(&todos, &args);
}

pub fn print_todo_table(todos: &[Todo], args: &LsArgs) {
    let mut tw = TabWriter::new(stdout());
    tw.set_header_row(&["ID", "Title", "Priority", "Status", "Due", "Completed At"]);

    for todo in todos {
        print_todo_table_row(args, &mut tw, todo);
    }
    tw.flush().unwrap();
}

fn print_todo_table_row(args: &LsArgs, tw: &mut TabWriter<std::io::Stdout>, todo: &Todo) {
    let status = match todo.status {
        TodoStatus::Done => style("Done ").green(),
        TodoStatus::InProgress => style("In Progress ").cyan(),
        TodoStatus::Pending => style("Pending ").yellow(),
        TodoStatus::Cancelled => style("Cancelled").red(),
    };
    let priority = match todo.priority {
        Priority::Low => style("Low").cyan(),
        Priority::Medium => style("Medium").green(),
        Priority::High => style("High").yellow().bold(),
        Priority::Critical => style("Critical").red().bold(),
    };

    tw.begin_row();
    tw.add_cell(&todo.id);
    tw.add_cell(&todo.title);
    tw.add_cell(&priority);
    tw.add_cell(&status);

    if let Some(due) = todo.due_at {
        tw.add_cell(&time_ago(due));
    } else {
        tw.add_cell(&"-");
    }

    if let Some(completed_at) = todo.completed_at
        && todo.status.is_done()
    {
        tw.add_cell(&time_ago(completed_at));
    } else {
        tw.add_cell(&"-");
    }

    if args.all {
        tw.push_header(&"Description");
        if let Some(ref desc) = todo.description {
            tw.add_cell(desc);
        } else {
            tw.add_cell(&"-");
        }

        tw.push_header(&"Tags".to_string());
        if !todo.tags.is_empty() {
            tw.add_cell(&todo.tags.join(", "));
        } else {
            tw.add_cell(&"-");
        }

        tw.push_header(&"Created At".to_string());
        tw.add_cell(&time_ago(todo.created_at));
    }
    tw.end_row().unwrap();
}
