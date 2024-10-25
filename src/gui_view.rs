use eframe::Frame;
use egui::{ Context };
use crate::db_repo::DbRepo;
use crate::gui_view::Tab::{CREATE, DELETE, HOME, SEARCH, STATUS, VIEW};
use crate::repository::Repository;

pub struct ToDoApp {
    repo: Box<dyn Repository>,
    tab: Tab
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
        Self { repo: repo2, tab: HOME }
    }
}

impl Default for ToDoApp {
    fn default() -> Self {
        Self {
            repo: Box::new(DbRepo::new("todo.db")),
            tab: HOME
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
                    ui.heading("Under Construction");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                }
                VIEW => {
                    ui.heading("Under Construction");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                }
                SEARCH => {
                    ui.heading("Under Construction");
                    if ui.button("back").clicked() {
                        self.tab = HOME
                    }
                }
                DELETE => {
                    ui.heading("Under Construction");
                    if ui.button("back").clicked() {
                        self.tab = HOME
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