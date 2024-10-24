use eframe::Frame;
use egui::Context;

#[derive(Default)]
pub struct ToDoApp {}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
        });
    }
}