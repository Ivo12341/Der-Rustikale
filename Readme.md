# ToDO Application

Welcome to the ToDO Application, a simple command-line tool to manage your tasks.
On the master branch is the main Comman line version of the file, that uses txt files for Storage.
On the "DB" branch there is a version that uses sqlite.

## Features

- **Create Task**: Add a new task with a title, due date, and priority.
- **View Tasks**: Display all tasks.
- **Search Tasks**: Search for tasks containing a specific term.
- **Delete Task**: Remove a task by its title.
- **Change Status**: Update the status of a task.

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/ToDO.git
    cd ToDO
    ```

2. **Build the project**:
    ```sh
    cargo build
    ```

## Usage

Run the application:
```sh
cargo run
```
Commands available:
```sh
create
```
Create a new task.
```sh
view
```
Display all tasks.
```sh
search
```
Search for a task.
```sh
delete
```
Delete a task.
```sh
status
```
Change the stauts (not started, working, done) of a task.
```sh
exit
```
Exit the program.