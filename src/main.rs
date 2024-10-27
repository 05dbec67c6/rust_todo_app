mod commands;
mod structs;

use std::io::stdin;
use crate::commands::{show_help, create_todo, list_todos, delete_todo};
use crate::structs::Todo;

fn main() {
    let mut note_list: Vec<Todo> = vec![];

    loop {
        greetings();
        if let Err(e) = process_user_command(&mut note_list) {
            println!("Error: {}", e);
        }
        println!("-----------------------------")
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
            if let Some(new_note) = new_note {
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
    println!("Enter your task. Enter 'help' for help");

    let a: i32 = 5;

    match a {
        5 => println!("its 5"),
        _ => (),
    }

    if let 5 = a { println!("its 5") }
}

