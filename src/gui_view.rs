use crate::db_repo::DbRepo;
use crate::gui_view::Tab::{CREATE, DELETE, HOME, SEARCH, STATUS, VIEW};
use crate::repository::Repository;
use crate::task::{Status, Task};
use eframe::Frame;
use egui::{popup_below_widget, Context, Id, PopupCloseBehavior, Response, Ui};

pub struct ToDoApp {
    repo: Box<dyn Repository>,
    tab: Tab,
    title_input: String,
    due_date_input: String,
    error: [String; 2],
    prio: i32,
    term: String,
    result: String,
    status: String,
}

enum Tab {
    HOME,
    CREATE,
    VIEW,
    SEARCH,
    DELETE,
    STATUS,
}

impl ToDoApp {
    pub fn new(repo2: Box<dyn Repository>) -> Self {
        Self {
            repo: repo2,
            tab: ToDoApp::default().tab,
            title_input: ToDoApp::default().title_input,
            due_date_input: ToDoApp::default().due_date_input,
            error: ToDoApp::default().error,
            prio: ToDoApp::default().prio,
            term: ToDoApp::default().term,
            result: ToDoApp::default().result,
            status: ToDoApp::default().status,
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
            result: String::from(""),
            status: String::from(""),
        }
    }
}

impl eframe::App for ToDoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.tab {
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
                    self.tab = HOME;
                    self.error[0] = String::from("Nich jültüsch");
                }
                let lulu = ui.label("Title");
                ui.text_edit_singleline(&mut self.title_input)
                    .labelled_by(lulu.id);
                let lulu2 = ui.label("Due Date (yyyy-mm-dd)");
                ui.text_edit_singleline(&mut self.due_date_input)
                    .labelled_by(lulu2.id);
                ui.colored_label(egui::Color32::RED, &self.error[0]);
                if !Task::match_date_format(&self.due_date_input) {
                    self.error[0] = String::from("Nich jültüsch");
                } else {
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
                    } else {
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
                } else {
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
                ui.colored_label(egui::Color32::RED, &self.error[0]);
                ui.label(&self.result);
                if ui.button("Search").clicked() {
                    if self.term != "" {
                        let res = self.repo.search_tasks(&self.term.trim());
                        self.result = String::from("");
                        if res != "" {
                            self.result = res;
                        } else {
                            self.result = String::from("No Task matching the term found");
                        }
                    } else {
                        self.error[0] = String::from("Please enter a keyword!")
                    }
                }
            }
            DELETE => {
                ui.heading("Delete Tasks");
                if ui.button("back").clicked() {
                    self.tab = HOME;
                    self.term = String::from("");
                    self.error[0] = String::from("");
                }
                self.title_combobox(ui);
                let popup_id = Id::new("popup_id");
                ui.colored_label(egui::Color32::RED, &self.error[0]);
                let delete_button = ui.button("Delete");
                self.success_popup(ui, popup_id, &delete_button, "Task successfully deleted!");
                if delete_button.clicked() {
                    self.error[0] = String::from("");
                    if self.term != "" {
                        self.repo.delete_tasks(&self.term);
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                        self.term = String::from("");
                    } else {
                        self.error[0] = String::from("Please enter a keyword!");
                    }
                }
            }
            STATUS => {
                ui.heading("Update the Status of Tasks");
                if ui.button("back").clicked() {
                    self.tab = HOME
                }
                let popup_id = Id::new("popup_id");
                self.title_combobox(ui);
                egui::ComboBox::from_label("Select Status")
                    .selected_text(&self.status)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.status,
                            String::from("Not Started"),
                            "Not Started",
                        );
                        ui.selectable_value(&mut self.status, String::from("Working"), "Working");
                        ui.selectable_value(&mut self.status, String::from("Done"), "Done");
                    });
                ui.colored_label(egui::Color32::RED, &self.error[0]);
                let update_button = ui.button("Update");
                self.success_popup(ui, popup_id, &update_button, "Status successfully updated");
                if update_button.clicked() {
                    if self.term != "" && self.status != "" {
                        self.repo.update_status(
                            &self.term,
                            Status::get_status_from_string(&self.status),
                        );
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                        self.term = String::from("");
                    } else if self.term == "" || self.status == "" {
                        self.error[0] = String::from("Enter a Status and a title.")
                    }
                }
            }
        });
    }
}

impl ToDoApp {
    fn title_combobox(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Select Task")
            .selected_text(&self.term)
            .show_ui(ui, |ui| {
                for title in self.repo.retrieve_tasks_title() {
                    ui.selectable_value(&mut self.term, String::from(&title), title);
                }
            });
    }

    fn success_popup(&mut self, ui: &mut Ui, popup_id: Id, button: &Response, msg: &str) {
        popup_below_widget(
            ui,
            popup_id,
            &button,
            PopupCloseBehavior::CloseOnClick,
            |ui| {
                ui.set_min_width(300.0);
                ui.label(msg);
                if ui.button("View the change").clicked() {
                    self.tab = VIEW;
                }
            },
        );
    }
}
