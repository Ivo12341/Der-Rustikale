use sqlite::Connection;
use crate::repository::Repository;
use crate::task::{Status, Task};

pub struct DbRepo {
    connection: Connection
}

impl DbRepo {
    pub(crate) fn new(db_path: &str) -> Self {
        let connection = sqlite::open(db_path).unwrap();
        DbRepo { connection }
    }

}

impl Repository for DbRepo {
    fn save_task(&self, task: &Task) -> bool {
        let save_query = format!("INSERT INTO TABLE TASK VALUES {}, {}, {}, {}", task.title, task.due_date, task.priority, Status::get_string_from_status(&task.status));
        match self.connection.execute(save_query) {
            Ok(_) => {true}
            Err(_) => {false}
        }
    }

    fn retrieve_tasks(&self) -> Vec<String> {
        todo!()
    }

    fn search_tasks(&self, term: &str) -> String {
        todo!()
    }

    fn delete_tasks(&self, term: &str) -> bool {
        todo!()
    }

    fn update_status(&self, term: &str, status: Status) -> bool {
        todo!()
    }
}
