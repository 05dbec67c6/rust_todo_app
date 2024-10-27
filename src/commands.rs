use std::io::stdin;
use crate::structs::Todo;

pub fn show_help() {
    println!("-Type \"help\" to show this help.");
    println!("-Type \"create\" to create a new todo.");
    println!("-Type \"list\" to list all todos.");
    println!("-Type \"delete\" to delete a new todo.");

}

pub fn create_todo() -> Result<Todo, String> {
    println!("Enter the text for the new todo:");
    let mut new_todo: String = String::new();
    stdin().read_line(&mut new_todo).map_err(|_| "Failed to read line".to_string())?;
    let new_todo = Todo::new(new_todo.trim());
    println!("New todo created.");
    Ok(new_todo)
}

pub fn list_todos(vec: &Vec<Todo>) {
    if vec.is_empty() {
        println!("There are no current todos.");
        return;
    }
    println!("Right now there are the following Todos:");
    for (index, element) in vec.iter().enumerate() {
        println!("{}. - {}", index + 1, element.todo_text)
    }
}

pub fn delete_todo(vec: &mut Vec<Todo>) {
    if vec.is_empty() {
        println!("There are no current todos.");
        return;
    }
    println!("Which todo do you want to delete?");
    list_todos(vec);
    let mut todo_to_delete = String::new();
    stdin().read_line(&mut todo_to_delete).expect("TODO: panic message");
    match todo_to_delete.trim().parse::<usize>() {
        Ok(index) => {
            if index == 0 || index > vec.len() {
                println!("Invalid index.")
            } else {
                vec.remove(index - 1);
                println!("Todo deleted");
            }
        }
        Err(_) => println!("Not a valid number.")
    }
}

