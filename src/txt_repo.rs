use std::fs::{create_dir_all, read_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use regex::Regex;
use crate::{ERR_GENERAL, ERR_OUTPUT};
use crate::repository::Repository;
use crate::task::{Status, Task};



pub struct TxtRepo {
    file_path: String,
}

impl Repository for TxtRepo {
    fn save_task(&self, task: &Task) -> bool {
        let result_string = Self::construct_result_string(task);
        let file_title = format!("{}", task.title.trim());
        if Path::exists(Path::new(&file_title)) {
            return false
        }
        let mut file: Option<File> = None;
        file = match File::create(format!("./db/{file_title}.txt")) {
            Ok(mut file) => {
                file.write_all(&result_string.into_bytes()).expect(ERR_OUTPUT);
                Some(file)
            }
            Err(_) => {
                None
            }
        };
        true
    }

    fn retrieve_tasks(&self) -> Vec<String> {
        let mut task_vec: Vec<String> = Vec::new();
        let paths = read_dir("./db").unwrap();
        for path in paths {
            let mut task_file = File::open(path.unwrap().path()).unwrap();
            task_vec.push(TxtRepo::get_file_contents(&mut task_file).trim().parse().unwrap());
        }
        task_vec
    }

    fn search_tasks(&self, term: &str) -> String {
        let mut return_string = String::from("");
        let leggie_from_lintendo = Regex::new(&format!(".*{}.*", regex::escape(&term.trim()))).unwrap();
        let paths = read_dir("./db").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            let mut task_file = File::open(&path).unwrap();
            let task = TxtRepo::get_file_contents(&mut task_file);
            if leggie_from_lintendo.is_match(&task) {
                return_string.push_str(&format!("{task}"));
            }
        }
        return_string
    }

    fn delete_tasks(&self, term: &str) -> bool {
        let file_path_str = format!("./db/{}.txt", &term.trim());
        if TxtRepo::check_path_exists(&file_path_str) {
            TxtRepo::remove_file(&file_path_str);
            true
        }
        else {
            false
        }
    }

    fn update_status(&self, term: &str, new_status: Status) -> bool {
        let file_path_str = format!("./db/{}.txt", &term.trim());
        if TxtRepo::check_path_exists(&file_path_str) {
            let mut file = OpenOptions::new().read(true).write(true).open(&file_path_str).expect(ERR_GENERAL);
            let cont = TxtRepo::get_file_contents(&mut file);
            let mut parts: Vec<&str> = cont.split("|").collect();
            let status_string = Status::get_string_from_status(&new_status);
            parts[3] = Box::leak(status_string.into_boxed_str());
            let updated_task = Task::construct_from_parts(&parts);
            let result_string = TxtRepo::construct_result_string(&updated_task);
            TxtRepo::clear_file_contents(&file_path_str);
            file.write_all(result_string.as_bytes()).expect(ERR_OUTPUT);
            true
        }
        else {
            false
        }
    }
}

impl TxtRepo {
    pub fn new(file_path: String) -> Self {
        create_dir_all("./db").expect("Hallo");
        TxtRepo { file_path }
    }

    fn construct_result_string(task: &Task) -> String {
        format!("{} | {} | {} | {:?}\n", task.title.trim(), task.due_date.trim(), task.priority, task.status)
    }

    pub fn check_path_exists(path_str: &str) -> bool {
        let file_path = Path::new(&path_str);
        Path::exists(file_path)
    }

    pub fn clear_file_contents(path_str: &str) {
        OpenOptions::new().write(true).truncate(true).open(path_str).expect("Error while opening file");
    }

    fn get_file_contents(file: &mut File) -> String {
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).expect(ERR_GENERAL);
        file_contents
    }

    fn remove_file(path_str: &str) -> bool {
        let file_path = Path::new(path_str);
        if file_path.exists() {
            match std::fs::remove_file(file_path) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}