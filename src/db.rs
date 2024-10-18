const CREATE_QUERY: &str = "CREATE TABLE IF NOT EXISTS tasks (title TEXT, due_date STRING, priority INTEGER, status STRING);";

pub fn start() {
    let connection = sqlite::open("todo.db").unwrap();
    connection.execute(CREATE_QUERY).unwrap();

}