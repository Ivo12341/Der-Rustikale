use crate::ERR_VALID_OPTION;
use std::io;
use crate::{ ERR_INPUT };
use crate::repository::Repository;
use crate::task::Task;
use crate::task::Status;

pub struct View {
    repo: Box<dyn Repository>
}

impl View {
    pub fn new(repo: Box<dyn Repository>) -> Self {
        View { repo }
    }

    pub fn start(&self) {
        println!("Welcome to the best ToDo Application EUW");
        loop {
            println!("What do?");
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect(ERR_INPUT);
            match command.as_str().trim() {
                "create" => Self::create_task(&self),
                "view" => Self::display_tasks(&self),
                "search" => Self::search_tasks(&self),
                "delete" => Self::delete_tasks(&self),
                "status" => Self::change_status(&self),
                "exit" => break,
                _ => println!("{}", command.as_str()),
            }
        }
    }

    fn create_task(&self) {
        let mut new_task = Task { title: String::new(), due_date: String::new(), priority: 0, status: Status::NotStarted };
        let mut temp_date = String::new();
        loop {
            println!("When is the Task due (yyyy-mm-dd)");
            io::stdin().read_line(&mut temp_date).expect(ERR_INPUT);
            if !Task::match_date_format(&temp_date) {
                println!("Wrong Format (use yyyy-mm-dd)");
            } else {
                new_task.due_date = temp_date.clone();
                break;
            }
        }
        println!("What Priority does it have? (i32 Number)");
        let mut temp_prio = String::new();
        io::stdin().read_line(&mut temp_prio).expect(ERR_INPUT);
        match temp_prio.trim().parse() {
            Ok(num) => new_task.priority = num,
            Err(..) => println!("Enter a valid i32 Number please :3"),
        }
        loop {
            println!("What is the Title of the task?");
            io::stdin().read_line(&mut new_task.title).expect(ERR_INPUT);
            if self.repo.save_task(&new_task) {
                break;
            } else {
                println!("Task already seems to exist.")
            }
        }
    }

    fn display_tasks(&self) {
        for task in self.repo.retrieve_tasks() {
            println!("{task}");
        }
    }

    fn search_tasks(&self) {
        println!("If you wish to exit type q");
        loop {
            println!("Enter search Term");
            let mut term = String::new();
            io::stdin().read_line(&mut term).expect(ERR_INPUT);
            if term.trim() == String::from("q") {
                break;
            }
            let tasks = self.repo.search_tasks(&term);
            if (tasks == "") {
                println!("No Tasks found matching your search");
            }
            else {
                println!("Tasks that matched your search:");
                println!("{tasks}");
            }
        }
    }

    fn delete_tasks(&self) {
        loop {
            println!("Enter Title of Task to be deleted");
            let mut term = String::new();
            io::stdin().read_line(&mut term).expect(ERR_INPUT);
            if (self.repo.delete_tasks(&term)) {
                break;
            }
            else {
                println!("Task with name not found. \n Task not deleted.");
            }
        }
    }

    fn change_status(&self) {
        println!("Enter Title of Task to be updated");
        let mut term = String::new();
        io::stdin().read_line(&mut term).expect(ERR_INPUT);
            loop {
                let mut status = String::new();
                println!("What status do you want to give 1: not started, 2: working, 3: done");
                io::stdin().read_line(&mut status).expect(ERR_INPUT);
                let new_status: Option<Status> = match status.trim() {
                    "1" => Some(Status::NotStarted),
                    "2" => Some(Status::Working),
                    "3" => Some(Status::Done),
                    _ => None,
                };
                match new_status {
                    None => {
                        println!("{ERR_VALID_OPTION}");
                    }
                    Some(new_status) => {
                        match self.repo.update_status(&term, new_status) {
                            true => break,
                            false => {
                                println!("Task not found");
                                self.change_status();
                            },
                        };

                    }
                }
            }
    }
}


