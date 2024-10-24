mod console_view;
mod task;
mod db_repo;
mod txt_repo;
mod repository;
mod gui_view;

use std::io;
use crate::db_repo::DbRepo;
use crate::txt_repo::TxtRepo;
use crate::console_view::ConsoleView;
use crate::gui_view::ToDoApp;
use crate::repository::Repository;

const ERR_GENERAL: &str = "Error occurred";
const ERR_INPUT: &str = "Failed to read line";
const ERR_OUTPUT: &str = "Failed to write";
const ERR_VALID_OPTION: &str = "Enter a valid option";

fn main() {
        loop {
            println!("Which view would you like to use, console or GUI");
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect(ERR_INPUT);
            match command.as_str().trim() {
                "console" => {
                    let repo = query_method();
                    let regular_view = ConsoleView::new(repo);
                    regular_view.start();
                },
                "GUI" => {
                    let repo = query_method();
                    let options = eframe::NativeOptions {
                        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
                        ..Default::default()
                    };
                    eframe::run_native(
                        "Hallo Zusammen",
                        options,
                        Box::new(|cc| {
                            Ok(Box::new(ToDoApp::new(repo)))
                        }),
                    ).expect("Crash and burn");
                }
                _ => println!("I'll be taking that one!")
            }
        }
}

fn query_method() -> Box<dyn Repository> {
    loop {
        println!("Which data retention method would you like to use? db, txt?");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect(ERR_INPUT);
        match command.as_str().trim() {
            "txt" => {
                return Box::new(TxtRepo::new());
            },
            "db" => {
                return Box::new(DbRepo::new("todo.db"));
            },
            "GUI" => {}
            _ => println!("Coffee, Cheetos, Chicken"),
        }
    }
}
