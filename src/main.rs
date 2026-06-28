use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;

use crate::adapters::cli::CliAdapter;
use crate::adapters::cli::use_cases::CliUseCases;
use crate::application::ports::todo_repository::TodoRepository;
use crate::application::usecases::todos::create_todo::CreateTodoUC;
use crate::application::usecases::todos::delete_all::DeleteAllTodosUC;
use crate::application::usecases::todos::delete_by_id::DeleteTodoByIdUC;
use crate::application::usecases::todos::get_all_todos::GetAllTodosUC;
use crate::infrastructure::repositories::sqlite::todo::TodoRepositorySqlite;

mod adapters;
mod application;
mod domain;
mod infrastructure;
mod shared;

#[tokio::main]
async fn main() {
    let db = SqlitePoolOptions::new()
        .max_connections(100)
        .connect("sqlite:doto.db")
        .await
        .expect("Failed to connect to database");

    // NOTE: Repositories
    let todo_repo: Arc<dyn TodoRepository> = Arc::new(TodoRepositorySqlite::new(db));

    // NOTE: Usecases
    let create_todo_uc = CreateTodoUC::new(Arc::clone(&todo_repo));
    let get_all_todos_uc = GetAllTodosUC::new(Arc::clone(&todo_repo));
    let delete_todo_by_id_uc = DeleteTodoByIdUC::new(Arc::clone(&todo_repo));
    let delete_all_todos_uc = DeleteAllTodosUC::new(todo_repo);

    // NOTE: Adapters
    let cli = CliAdapter::new(CliUseCases::new(
        create_todo_uc,
        get_all_todos_uc,
        delete_todo_by_id_uc,
        delete_all_todos_uc,
    ));
    cli.run().await;
}
