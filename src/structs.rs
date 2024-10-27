#[derive(Debug)]
pub(crate) struct Todo {
    pub(crate) todo_text: String,
}

impl Todo {
    pub(crate) fn new(todo_text: String) -> Todo {
        Todo { todo_text }
    }
}