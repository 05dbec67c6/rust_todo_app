mod commands;
mod structs;

use crate::commands::{create_todo, delete_todo, list_todos, show_help};
use crate::structs::Todo;
use std::io::stdin;

fn main() {
    let mut note_list: Vec<Todo> = vec![];

    loop {
        greetings();
        if let Err(e) = process_user_command(&mut note_list) {
            println!("Error: {}", e);
        }
        println!("-----------------------------");
    }
}

fn process_user_command(note_list: &mut Vec<Todo>) -> Result<(), String> {
    let mut command = String::new();
    stdin().read_line(&mut command).expect("Couldn't read line");

    match command.trim() {
        "help" => {
            show_help();
            Ok(())
        }
        "create" => {
            let new_note = create_todo();
            if let Ok(new_note) = new_note {
                note_list.push(new_note);
            }
            Ok(())
        }
        "list" => {
            list_todos(note_list);
            Ok(())
        }
        "delete" => {
            delete_todo(note_list);
            Ok(())
        }
        _ => {
            println!("Not a valid command");
            Ok(())
        }
    }
}

fn greetings() {
    println!("[Enter your task. Enter 'help' for help.]");
}

