use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use regex::Regex;
use crate::Status::NotStarted;

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
            status: Status::NotStarted,
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

fn main() {
    let exit = false;
    println!("Welcome to the best ToDo Application EUW");
    if !Path::new("db.txt").exists() {
        File::create("db.txt").expect("Failed to create file maybe check permissions.");
    }
    while !exit {
        let mut db_file = OpenOptions::new().read(true).write(true).append(true).open("db.txt").expect("Failed to open file permissions");
        println!("What do?");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        match command.as_str().trim() {
            "create" => {
                let taggus = create_task();
                save_task_to_file(&mut db_file, taggus);
            }
            "view" => display_tasks(&mut db_file),
            "search" => search_tasks(&mut db_file),
            "delete" => delete_tasks(&mut db_file),
            "dbg" => dbg(&mut db_file),
            "status" => change_status(&mut db_file),
            _ => println!("{}", command.as_str()),
        }
    }
}

fn create_task() -> Task {
    let mut new_task = Task { title: String::new(), due_date: String::new(), priority: 0, status: Status::NotStarted };
    println!("What is the Title of the task?");
    io::stdin().read_line(&mut new_task.title).expect("Failed to read line");
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
    new_task
}

fn save_task_to_file(file: &mut File, task: Task) {
    let result_string = format!("{} | {} | {} | {:?}\n", task.title.trim(), task.due_date.trim(), task.priority, task.status);
    file.write_all(&result_string.as_bytes()).expect("Okie Dokie");
}

fn get_file_contents(file: &mut File)-> String {
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Error");
    file_contents
}

fn display_tasks(file: &mut File) {
    println!("Taks:");
    let cont = get_file_contents(file);
    let tasks: Vec<&str> = cont.split("\n").collect();
    for task in tasks {
        println!("{task}");
    }
}

fn search_tasks(file: &mut File) {
    let cont = get_file_contents(file);
    let tasks: Vec<&str> = cont.split("\n").collect();
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
        for task in tasks.clone() {
            if leggie_from_lintendo.is_match(task) {
                println!("{task}");
            }
        }
    }
}

fn delete_tasks(file: &mut File) {
    let cont = get_file_contents(file);
    let tasks: Vec<&str> = cont.split('\n').collect();
    let mut filtered_tasks: Vec<&str> = tasks.clone();
    println!("Enter Title of Task to be deleted");
    let mut term = String::new();
    io::stdin().read_line(&mut term).expect("Failed to read line");
    for task in tasks.clone() {
        let parts: Vec<&str> = task.split('|').collect();
        if parts[0].replace('|', "").trim().to_lowercase().contains(&term.trim().to_lowercase()) {
            filtered_tasks.retain(|&task| !task.replace('|', "").trim().to_lowercase().contains(&term.trim().to_lowercase()));
            let write_string = construct_write_string(&filtered_tasks);
            let mut file = OpenOptions::new().write(true).truncate(true).open("db.txt").expect("Failed to open file");
            file.write_all(write_string.as_bytes()).unwrap();
        }
    }
}

fn change_status(file: &mut File) {
    let cont = get_file_contents(file);
    let tasks: Vec<&str> = cont.split('\n').collect();
    let mut filtered_tasks: Vec<String> = tasks.iter().map(|&s| s.to_string()).collect();
    println!("Enter Title of Task to be updated");
    let mut term = String::new();
    io::stdin().read_line(&mut term).expect("Failed to read line");
    for task in tasks.clone() {
        let parts: Vec<&str> = task.split('|').collect();
        if parts[0].replace('|', "").trim().to_lowercase().contains(&term.trim().to_lowercase()) {
            let mut updated_status = None;
            loop {
                println!("What Status do you want to give it?");
                let mut stat = String::new();
                io::stdin().read_line(&mut stat).expect("Failed to read line");
                updated_status = match stat.trim().to_lowercase().as_str() {
                    "not started" => Some(Status::NotStarted),
                    "working" => Some(Status::Working),
                    "done" => Some(Status::Done),
                    _ => None
                };
                match updated_status {
                    None => { println!("Enter a valid option ty.") }
                    Some(_) => { break }
                }
            }
            filtered_tasks.retain(|task| !task.replace('|', "").trim().to_lowercase().contains(&term.trim().to_lowercase()));
            for task in &filtered_tasks {
                println!("{task}");
            }
            let part_task = Task::construct_from_parts(&parts);
            let updated_task = Task {
                status: updated_status.unwrap(),
                ..part_task
            };
            let formatted_task = format!("{} | {} | {} | {:?}", updated_task.title, updated_task.due_date, updated_task.priority, updated_task.status);
            filtered_tasks.push(formatted_task);
        }
    }
    let write_string = construct_write_string(&filtered_tasks.iter().map(|s| s.as_str()).collect());
    let mut file = OpenOptions::new().write(true).truncate(true).open("db.txt").expect("Failed to open file");
    file.write_all(write_string.as_bytes()).unwrap();
}

fn construct_write_string(tasks: &Vec<&str>) -> String {
    let mut write_string = String::new();
    for task in tasks.iter().filter(|&&task| !task.trim().is_empty()) {
        write_string.push_str(task.trim());
        write_string.push_str("\n");
    }
    write_string
}

fn dbg(file: &mut File) {
    let cont = get_file_contents(file);
    let tasks: Vec<&str> = cont.split('\n').collect();
    let mut filtered_tasks: Vec<&str> = tasks.clone();
    //println!("Enter Title of Task to be updated");
    let mut term = String::new();
    //io::stdin().read_line(&mut term).expect("Failed to read line");
    for task in tasks.clone() {
        let parts: Vec<&str> = task.split('|').collect();
        for part in parts {
            println!("{part}");
        }
    }
}
