mod commands;
mod structs;

use std::io::stdin;
use crate::commands::{show_help, create_note, list_notes};
use crate::structs::Todo;

fn main() {
    let mut note_list: Vec<Todo> = vec![];

    loop {
        greetings();
        process_user_command(&mut note_list);
    }
}

fn process_user_command(note_list: &mut Vec<Todo>) {
    let mut command = String::new();
    stdin().read_line(&mut command).expect("TODO: panic message");

    match command.trim() {
        "help" => {
            show_help()
        }
        "create" => {
            {
                let new_note = create_note();
                match new_note {
                    Some(new_note) => note_list.push(new_note),
                    None => (),
                }
            }
        }
        "list" => {
            list_notes(note_list)
        }
        "delete" => {}
        _ => println!("Not a valid command")
    }
}

fn greetings() {
    println!("Enter your task. Enter 'help' for help");
}
