mod console_view;
mod task;
mod db_repo;
mod txt_repo;
mod repository;
mod gui_view;

use std::io;
use crate::db_repo::DbRepo;
use crate::txt_repo::TxtRepo;
use crate::console_view::View;
use crate::gui_view::ToDoApp;

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
            "GUI" => {
                let options = eframe::NativeOptions {
                    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
                    ..Default::default()
                };
                eframe::run_native(
                    "Hallo Zusammen",
                    options,
                    Box::new(|cc| {
                        Ok(Box::<ToDoApp>::default())
                    }),
                ).expect("Crash and burn");
            }
            "exit" => break,
            _ => println!("Coffee, Cheetos, Chicken"),
        }
    }
}
