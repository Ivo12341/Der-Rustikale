mod db;
mod txt;
mod task;

use std::io;

const ERR_GENERAL: &str = "Error occurred";
const ERR_INPUT: &str = "Failed to read line";
const ERR_OUTPUT: &str = "Failed to write";
const ERR_VALID_TASK: &str = "Enter a valid task";
const ERR_VALID_OPTION: &str = "Enter a valid option";

fn main() {
    loop {
        println!("Which version of the program would you like to use? db or txt or exit?");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect(ERR_INPUT);
        match command.as_str().trim() {
            "txt" => txt::start(),
            "db" => db::start(),
            "exit" => break,
            _ => println!("Coffee, Cheetos, Chicken"),
        }
    }
}
