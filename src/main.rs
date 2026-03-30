//Code by Gwen Thelin <3
use crossterm::event::{self, Event, KeyCode};
use crossterm::{cursor, queue};
use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;
//Add to this as needed

struct SessionData {
    is_empty: bool,
    is_single_response: bool,
    should_quit: bool,
}

impl SessionData {
    fn swap_emptiness(&mut self) {
        self.is_empty = !self.is_empty;
    }
}

fn main() {
    let mut session = SessionData {
        is_empty: false,
        is_single_response: false,
        should_quit: false,
    };

    clear_screen();

    println!("Welcome to the decider!");

    println!("Do you want a (s)ingle choice or a (r)anking?");
    io::stdout().flush().unwrap();

    loop {
        match event::read().unwrap() {
            Event::Key(key_event) => {
                if let KeyCode::Char('s') = key_event.code {
                    handle_choice(1, &mut session);
                    break;
                } else if let KeyCode::Char('r') = key_event.code {
                    handle_choice(2, &mut session);
                    break;
                } else {
                    println!("Invalid selection. Please enter \"s\" or \"r\".");
                    session.should_quit = true;
                    break;
                }
            }
            _ => {}
        }
    }

    //A sleep call to make the program feel more dynamic and human
    thread::sleep(Duration::from_secs(1));

    clear_screen();

    let mut iteration: u8 = 1;

    if !session.should_quit {
        for option in input_loop(&mut session).iter() {
            if iteration == 1 {
                clear_screen();
            }

            if !session.is_empty {
                if !session.is_single_response {
                    print!("{iteration}. ");
                }

                print!("{option}");
                iteration += 1;
                println!();

                if session.is_single_response {
                    break;
                }
            }
        }
    }

    print!("Goodbye!");
}

fn handle_choice(choice: u32, session: &mut SessionData) {
    match choice {
        1 => {
            session.is_single_response = true;
        }
        2 => {
            session.is_single_response = false;
        }
        _ => {
            eprintln!("Invalid selection. Please enter \"s\" or \"r\".");
        }
    }
    queue!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
}

fn clear_screen() {
    println!("\x1B[2J");
    println!("\x1B[H");
}

//I'm using a set so I don't have to import random & because it's quick
fn input_loop(session: &mut SessionData) -> HashSet<String> {
    let mut options = HashSet::new();
    loop {
        println!("type \"exit\" to exit");
        println!("Write an option:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input: String = input.trim().to_string();
        println!();

        if trimmed_input.contains("exit") {
            if options.is_empty() {
                options.insert("".to_string());
                if !session.is_empty {
                    session.swap_emptiness();
                }
            } else {
                if session.is_empty {
                    session.swap_emptiness();
                }
            }
            println!();
            thread::sleep(Duration::from_secs(2));
            return options;
        } else {
            options.insert(trimmed_input);
        }
        clear_screen();
    }
}
