use eframe::Frame;
use egui::{ Context };
use crate::db_repo::DbRepo;
use crate::gui_view::Tab::{CREATE, DELETE, HOME, SEARCH, STATUS, VIEW};
use crate::repository::Repository;
use crate::task::{Status, Task};

pub struct ToDoApp {
    repo: Box<dyn Repository>,
    tab: Tab,
    title_input: String,
    due_date_input: String,
    error: [String; 2],
    prio: i32,
    term: String,
    result: String
}

enum Tab {
    HOME,
    CREATE,
    VIEW,
    SEARCH,
    DELETE,
    STATUS
}

impl ToDoApp {
    pub fn new(repo2: Box<dyn Repository>) -> Self {
        Self { repo: repo2,
            tab: ToDoApp::default().tab,
            title_input: ToDoApp::default().title_input,
            due_date_input: ToDoApp::default().due_date_input,
            error: ToDoApp::default().error,
            prio: ToDoApp::default().prio,
            term: ToDoApp::default().term,
            result: ToDoApp::default().result
        }
    }
}

impl Default for ToDoApp {
    fn default() -> Self {
        Self {
            repo: Box::new(DbRepo::new("todo.db")),
            tab: HOME,
            title_input: String::from(""),
            due_date_input: String::from(""),
            error: [String::from(""), String::from("")],
            prio: 0,
            term: String::from(""),
            result: String::from("")
        }
    }
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                HOME => {
                    if ui.button("Create").clicked() {
                        self.tab = CREATE;
                    }
                    if ui.button("View").clicked() {
                        self.tab = VIEW;
                    }
                    if ui.button("Search").clicked() {
                        self.tab = SEARCH;
                    }
                    if ui.button("Delete").clicked() {
                        self.tab = DELETE;
                    }
                    if ui.button("Status").clicked() {
                        self.tab = STATUS;
                    }
                    }
                CREATE => {
                    ui.heading("Create a Task");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                    let lulu = ui.label("Title");
                    ui.text_edit_singleline(&mut self.title_input).labelled_by(lulu.id);
                    let lulu2 = ui.label("Due Date (yyyy-mm-dd)");
                    ui.text_edit_singleline(&mut self.due_date_input).labelled_by(lulu2.id);
                    ui.colored_label(egui::Color32::RED, &self.error[0]);
                    if !Task::match_date_format(&self.due_date_input) {
                        self.error[0] = String::from("Nich jültüsch");
                    }
                    else {
                        self.error = ToDoApp::default().error
                    }
                    ui.add(egui::Slider::new(&mut self.prio, 0..=100).text("Priorität"));
                    ui.colored_label(egui::Color32::RED, &self.error[1]);
                    if ui.button("Speichern").clicked() {
                        let task = Task {
                            title: self.title_input.to_string(),
                            due_date: self.due_date_input.to_string(),
                            priority: self.prio,
                            status: Status::NotStarted,
                        };
                        if self.repo.save_task(&task) {
                            self.error[1] = String::from("");
                            self.tab = HOME
                        }
                        else {
                            self.error[1] = String::from("Task already exists");
                        }
                    }
                }
                VIEW => {
                    ui.heading("View all Tasks");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                    let tasks = self.repo.retrieve_tasks();
                    if tasks.len() == 0 {
                        ui.label("No Tasks found");
                    }
                    else {
                        ui.label("Tasks: ");
                    }
                    for task in tasks {
                        ui.label(task);
                    }
                }
                SEARCH => {
                    ui.heading("Search Tasks");
                    if ui.button("back").clicked() {
                        self.tab = HOME;
                        self.term = String::from("");
                    }
                    let lala = ui.label("Search-term");
                    ui.text_edit_singleline(&mut self.term).labelled_by(lala.id);
                    ui.label(&self.result);
                    if ui.button("Search").clicked() {
                        let res = self.repo.search_tasks(&self.term.trim());
                        self.result = String::from("");
                        if res != "" {
                          self.result = res;
                        }
                        else {
                            self.result = String::from("No Task matching the term found");
                        }
                    }
                }
                DELETE => {
                    ui.heading("Delete Tasks");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                    ui.text_edit_singleline(&mut self.term);
                    if ui.button("Löschen").clicked() && self.term != "" {
                        self.repo.delete_tasks(&self.term);
                    }
                }
                STATUS => {
                    ui.heading("Under Construction");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                }
            }
        });
    }
}