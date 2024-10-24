use eframe::Frame;
use egui::Context;
use crate::db_repo::DbRepo;
use crate::repository::Repository;

pub struct ToDoApp {
    repo: Box<dyn Repository>
}

impl ToDoApp {
    pub fn new(repo2: Box<dyn Repository>) -> Self {
        Self { repo: repo2 }
    }
}

impl Default for ToDoApp {
    fn default() -> Self {
        Self {
            repo: Box::new(DbRepo::new("todo.db"))
        }
    }
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
        });
    }
}