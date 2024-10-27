use std::io::stdin;
use crate::structs::Todo;

pub fn show_help() {
    println!("-Type \"help\" to show this help.");
    println!("-Type \"create\" to create a new note.");
}

pub fn create_note() -> Option<Todo> {
    println!("Enter the text for the new todo:");
    let mut new_note = String::new();
    stdin().read_line(&mut new_note).expect("TODO: panic message");
    let new_note = Todo::new(new_note.trim().to_string());
    Some(new_note)
}

pub fn list_notes(vec: &Vec<Todo>) {
    println!("Right now there are the following Todos:");
    for (index, element) in vec.iter().enumerate() {
        println!("{}. - {}", index + 1, element.todo_text)
    }
}

pub fn delete_note(vec: &Vec<Todo>) {
    // let new_note = super::structs::Todo { todo_text: "csie".to_string() };
    println!("this is the CREATE")
}

