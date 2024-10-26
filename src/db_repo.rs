use crate::repository::Repository;
use crate::task::{Status, Task};
use crate::ERR_GENERAL;
use sqlite::Connection;

pub struct DbRepo {
    connection: Connection,
}

impl DbRepo {
    pub(crate) fn new(db_path: &str) -> Self {
        let create_query = "CREATE TABLE IF NOT EXISTS TASK (title TEXT, due_date STRING, priority INTEGER, status STRING);";
        let connection = sqlite::open(db_path).unwrap();
        connection.execute(create_query).expect(ERR_GENERAL);
        DbRepo { connection }
    }
}

impl Repository for DbRepo {
    fn save_task(&self, task: &Task) -> bool {
        let save_query = format!(
            "INSERT INTO TASK (title, due_date, priority, status) VALUES ('{}', '{}', {}, '{}')",
            task.title,
            task.due_date,
            task.priority,
            Status::get_string_from_status(&task.status)
        );
        match self.connection.execute(save_query) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn retrieve_tasks(&self) -> Vec<String> {
        let retrieve_query = "SELECT * FROM TASK";
        let mut statement = self.connection.prepare(retrieve_query).expect(ERR_GENERAL);
        let mut task_vec: Vec<String> = Vec::new();
        while let sqlite::State::Row = statement.next().unwrap() {
            let title: String = statement.read(0).unwrap();
            let due_date: String = statement.read(1).unwrap();
            let priority: i32 = statement.read::<i64, _>(2).unwrap() as i32;
            let status: String = statement.read(3).unwrap();
            let task = format!(
                "{} | {} | {} | {}",
                title.trim(),
                due_date.trim(),
                priority,
                status.trim()
            );
            task_vec.push(task);
        }

        task_vec
    }

    fn retrieve_tasks_title(&self) -> Vec<String> {
        let retrieve_query = "SELECT title FROM TASK";
        let mut statement = self.connection.prepare(retrieve_query).expect(ERR_GENERAL);
        let mut task_vec: Vec<String> = Vec::new();
        while let sqlite::State::Row = statement.next().unwrap() {
            let title: String = statement.read(0).unwrap();
            task_vec.push(title);
        }
        task_vec
    }

    fn search_tasks(&self, term: &str) -> String {
        let search_query = format!("SELECT * FROM TASK WHERE title LIKE '%{term}%'");
        let mut statement = self.connection.prepare(search_query).expect(ERR_GENERAL);
        let mut task_str: String = String::new();
        while let sqlite::State::Row = statement.next().unwrap() {
            let title: String = statement.read(0).unwrap();
            let due_date: String = statement.read(1).unwrap();
            let priority: i32 = statement.read::<i64, _>(2).unwrap() as i32;
            let status: String = statement.read(3).unwrap();
            let task = format!("{} | {} | {} | {}", title, due_date, priority, status);
            task_str.push_str(&task);
            task_str.push('\n');
        }
        task_str
    }

    fn delete_tasks(&self, term: &str) -> bool {
        let delete_query = format!("DELETE FROM TASK WHERE title LIKE '{term}'");
        match self.connection.execute(delete_query) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn update_status(&self, term: &str, status: Status) -> bool {
        let update_query = format!(
            "UPDATE TASK SET status = '{}' WHERE title LIKE '%{term}%'",
            { Status::get_string_from_status(&status) }
        );
        match self.connection.execute(update_query) {
            Ok(_) => true,
            Err(err) => {
                println!("{err}");
                false
            }
        }
    }
}
