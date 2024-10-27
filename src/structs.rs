#[derive(Debug)]
pub(crate) struct Todo {
    pub(crate) todo_text: String,
}

impl Todo {
    pub(crate) fn new(todo_text: &str) -> Todo {
        Todo { todo_text: todo_text.to_string() }
    }
}