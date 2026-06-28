use std::sync::Arc;

use crate::application::usecases::todos::{
    create_todo::CreateTodoUC, delete_all::DeleteAllTodosUC, delete_by_id::DeleteTodoByIdUC,
    get_all_todos::GetAllTodosUC,
};

pub(crate) struct CliUseCases {
    pub todos: TodoUCs,
}

pub(crate) struct TodoUCs {
    pub create_todo: CreateTodoUC,
    pub get_all_todos: GetAllTodosUC,
    pub delete_todo_by_id: DeleteTodoByIdUC,
    pub delete_all_todos_by_status: DeleteAllTodosUC,
}

impl CliUseCases {
    pub fn new(
        create_todo: CreateTodoUC,
        get_all_todos: GetAllTodosUC,
        delete_todo_by_id: DeleteTodoByIdUC,
        delete_all_todos: DeleteAllTodosUC,
    ) -> Self {
        Self {
            todos: TodoUCs {
                create_todo,
                get_all_todos,
                delete_todo_by_id,
                delete_all_todos_by_status: delete_all_todos,
            },
        }
    }
}
