use std::fs::{create_dir_all, read_dir, remove_file, File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use regex::Regex;
use crate::Status::{Done, NotStarted, Working};

#[derive(Debug)]
struct Task {
    title: String,
    due_date: String,
    priority: i32,
    status: Status,
}

impl Task {
    fn construct_from_parts(parts: &Vec<&str>) -> Task {
        let new_task = Task {
            title: parts[0].trim().parse().unwrap(),
            due_date: parts[1].trim().parse().unwrap(),
            priority: parts[2].trim().parse().unwrap(),
            status: Status::get_status_from_string(parts[3].trim()),
        };
        new_task
    }
}

#[derive(Debug)]
enum Status {
    NotStarted,
    Working,
    Done,
}

impl Status {
    fn get_string_from_status(status: Status) -> String {
        match status {
            NotStarted => {String::from("NotStarted")}
            Working => {String::from("Working")}
            Done => {String::from("Done")}
        }
    }

    fn get_status_from_string(string: &str) -> Status {
        match string {
            "NotStarted" => NotStarted,
            "Working" => Working,
            "Done" => Done,
            _ => NotStarted,
        }
    }
}

fn main() {
    println!("Welcome to the best ToDo Application EUW");
    create_dir_all("./db").expect("Hallo");
    loop {
        println!("What do?");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        match command.as_str().trim() {
            "create" => create_task(),
            "view" => display_tasks(),
            "search" => search_tasks(),
            "delete" => delete_tasks(),
            "status" => change_status(),
            "exit" => break,
            _ => println!("{}", command.as_str()),
        }
    }
}

fn create_task() {
    let mut new_task = Task { title: String::new(), due_date: String::new(), priority: 0, status: Status::NotStarted };
    let reggie_from_nintendo = Regex::new(r"\d{4}-\d{2}-\d{2}");
    let mut temp_date = String::new();
    loop {
        println!("When is the Task due (yyyy-mm-dd)");
        io::stdin().read_line(&mut temp_date).expect("Failed to read line");
        if !reggie_from_nintendo.clone().unwrap().is_match(&temp_date) {
            println!("Wrong Format (use yyyy-mm-dd)");
        }
        else {
            new_task.due_date = temp_date.clone();
            break;
        }
    }
    println!("What Priority does it have? (i32 Number)");
    let mut temp_prio = String::new();
    io::stdin().read_line(&mut temp_prio).expect("Failed to read line");
    match temp_prio.trim().parse() {
        Ok(num) => new_task.priority = num,
        Err(..) => println!("Enter a valid i32 Number please :3"),
    }
    loop {
        println!("What is the Title of the task?");
        io::stdin().read_line(&mut new_task.title).expect("Failed to read line");
        if save_task_in_new_file(&new_task) {
            break;
        }
    }
}

fn save_task_in_new_file(task: &Task) -> bool {
        let result_string = construct_result_string(task);
        let file_title = format!("{}", task.title.trim());
        let mut file: Option<File> = None;
        if !Path::exists(Path::new(&file_title)) {
            file = match File::create(format!("./db/{file_title}.txt")) {
                Ok(mut file) => {
                    file.write_all(&result_string.into_bytes()).expect("Failed to write");
                    Some(file)
                }
                Err(err) => {
                    println!("Failed to create File {err}");
                    None
                }
            };
        }
        match file {
            None => {
                println!("Task with that title already exists (must be unique)");
                false
            }
            Some(_) => {
                true
            }
        }
}

fn get_file_contents(file: &mut File)-> String {
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Error");
    file_contents
}

fn display_tasks() {
    println!("Tasks:");
    let paths = read_dir("./db").unwrap();
    for path in paths {
        let mut task_file = File::open(path.unwrap().path()).unwrap();
        let task = get_file_contents(&mut task_file);
        println!("{task}");
    }
}

fn search_tasks() {
    println!("If you wish to exit type q");
    loop {
        println!("Enter search Term");
        let mut term = String::new();
        io::stdin().read_line(&mut term).expect("Failed to read line");
        if term.trim() == String::from("q") {
            break;
        }
        let leggie_from_lintendo = Regex::new(&format!(".*{}.*", regex::escape(&term.trim()))).unwrap();
        println!("Tasks, that contain the term:");
        let paths = read_dir("./db").unwrap();
        let mut found = false;
        for path in paths {
            let path = path.unwrap().path();
            let mut task_file = File::open(&path).unwrap();
            let task = get_file_contents(&mut task_file);
            if leggie_from_lintendo.is_match(&task) {
                println!("{task}");
                found = true;
            }
        }
        if !found {
            println!("No tasks found containing the term.");
        }
    }
}

fn delete_tasks(){
    loop {
        println!("Enter Title of Task to be deleted");
        let mut term = String::new();
        io::stdin().read_line(&mut term).expect("Failed to read line");
        let file_path_str = format!("./db/{}.txt", &term.trim());
        let file_path = Path::new(&file_path_str);
        if Path::exists(file_path) {
            match remove_file(file_path) {
                Ok(_) => break,
                Err(_) => println!("File / Task not found enter a valid Title."),
            };
        }
    }
}

fn change_status() {
    println!("Enter Title of Task to be updated");
    let mut term = String::new();
    io::stdin().read_line(&mut term).expect("Failed to read line");
    let file_path_str = format!("./db/{}.txt", &term.trim());
    let file_path = Path::new(&file_path_str);
    if Path::exists(file_path) {
        let mut file = OpenOptions::new().read(true).open(&file_path_str).expect("Error while opening file");
        let cont = get_file_contents(&mut file);
        let mut parts: Vec<&str> = cont.split("|").collect();

        loop {
            let mut status = String::new();
            println!("What status do you want to give 1: not started, 2: working, 3: done");
            io::stdin().read_line(&mut status).expect("Error reading line");
            let new_status: Option<Status> = match status.trim() {
                "1" => Some(NotStarted),
                "2" => Some(Working),
                "3" => Some(Done),
                _ => None,
            };
            match new_status {
                None => {
                    println!("Please enter a valid option 1, 2, 3");
                }
                Some(new_status) => {
                    let status_string = Status::get_string_from_status(new_status);
                    parts[3] = Box::leak(status_string.into_boxed_str());
                    let updated_task = Task::construct_from_parts(&parts);
                    let result_string = construct_result_string(&updated_task);
                    let mut file = OpenOptions::new().write(true).truncate(true).open(&file_path_str).expect("Error while opening file");
                    file.write_all(result_string.as_bytes()).expect("Error writing");
                    break;
                }
            }
        }
    }
}

fn construct_result_string(task: &Task) -> String {
    format!("{} | {} | {} | {:?}\n", task.title.trim(), task.due_date.trim(), task.priority, task.status)
}
