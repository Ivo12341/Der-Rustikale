mod view;
mod task;
mod db_repo;
mod txt_repo;
mod repository;

use std::io;
use crate::db_repo::DbRepo;
use crate::txt_repo::TxtRepo;
use crate::view::View;

const ERR_GENERAL: &str = "Error occurred";
const ERR_INPUT: &str = "Failed to read line";
const ERR_OUTPUT: &str = "Failed to write";
const ERR_VALID_OPTION: &str = "Enter a valid option";

fn main() {
    loop {
        println!("Which version of the program would you like to use? db or txt or exit?");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect(ERR_INPUT);
        match command.as_str().trim() {
            "txt" => {
                let txt_repo = TxtRepo::new();
                let view_txt = View::new(Box::new(txt_repo));
                view_txt.start();
            },
            "db" => {
                let db_repo = DbRepo::new("todo.db");
                let view_db = View::new(Box::new(db_repo));
                view_db.start();
            },
            "exit" => break,
            _ => println!("Coffee, Cheetos, Chicken"),
        }
    }
}
