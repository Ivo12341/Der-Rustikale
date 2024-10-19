use crate::task::{Status, Task};

pub trait Repository {
    fn save_task(&self, task: &Task) -> bool;

    fn retrieve_tasks(&self) -> Vec<String>;

    fn search_tasks(&self, term: &str) -> String;

    fn delete_tasks(&self, term: &str) -> bool;

    fn update_status(&self, term: &str, status: Status) -> bool;
}