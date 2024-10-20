use regex::Regex;

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub due_date: String,
    pub priority: i32,
    pub status: Status,
}

impl Task {
    pub fn construct_from_parts(parts: &Vec<&str>) -> Task {
        let new_task = Task {
            title: parts[0].trim().parse().unwrap(),
            due_date: parts[1].trim().parse().unwrap(),
            priority: parts[2].trim().parse().unwrap(),
            status: Status::get_status_from_string(parts[3].trim()),
        };
        new_task
    }

    pub fn match_date_format(date: &str) -> bool {
        let reggie_from_nintendo = Regex::new(r"\d{4}-\d{2}-\d{2}");
        reggie_from_nintendo.clone().unwrap().is_match(date.trim())
    }
}

#[derive(Debug)]
pub enum Status {
    NotStarted,
    Working,
    Done,
}

impl Status {
    pub fn get_string_from_status(status: &Status) -> String {
        match status {
            Status::NotStarted => {String::from("NotStarted")}
            Status::Working => {String::from("Working")}
            Status::Done => {String::from("Done")}
        }
    }

    pub fn get_status_from_string(string: &str) -> Status {
        match string {
            "NotStarted" => Status::NotStarted,
            "Working" => Status::Working,
            "Done" => Status::Done,
            _ => Status::NotStarted,
        }
    }
}
