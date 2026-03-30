use std::collections::HashSet;
use std::io;
use std::thread;
use std::time::Duration;

struct SessionData {
    is_empty: bool,
}

impl SessionData {
    fn swap_emptiness(&mut self) {
        self.is_empty = !self.is_empty;
    }
}

fn main() {
    let mut session = SessionData { is_empty: false };

    println!("Welcome to the decider!");
    //A sleep call to make the program feel more dynamic and human
    thread::sleep(Duration::from_secs(1));
    //TODO: Tell user to type "exit" to exit
    let mut iteration: u8 = 1;

    for option in input_loop(&mut session).iter() {
        if !session.is_empty {
            print!("{iteration}. ");
            print!("{option}");
            iteration += 1;
            println!();
        }
    }

    if session.is_empty {
        print!("Goodbye!");
    }
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
    }
}
