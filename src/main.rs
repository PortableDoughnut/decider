use std::collections::HashSet;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Welcome to the decider!");
    //A sleep call to make the program feel more dynamic and human
    thread::sleep(Duration::from_secs(1));
    //TODO: Tell user to type "exit" to exit
    let mut iteration: u8 = 1;
    for option in input_loop().iter() {
        print!("{iteration}. ");
        print!("{option}");
        iteration += 1;
        println!();
    }
}

//I'm using a set so I don't hae to import random & because it's quick
fn input_loop() -> HashSet<String> {
    let mut options = HashSet::new();
    loop {
        println!("Write an option:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input: String = input.trim().to_string();

        //TODO: Cover the case of immedite exit
        if trimmed_input.contains("exit") {
            println!();
            thread::sleep(Duration::from_secs(2));
            return options;
        } else {
            options.insert(trimmed_input);
        }
    }
}
