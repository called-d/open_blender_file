use eframe::egui::{self, ViewportBuilder, ViewportId};

use crate::version_checker::BlenderVersion;
mod about;

struct MyApp {
    version: Box<BlenderVersion>,
    current_file: Box<str>,
    // state
    show_about: bool,
}

impl MyApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        version: Box<BlenderVersion>,
        current_file: Box<str>,
    ) -> Self {
        Self {
            version,
            current_file,
            show_about: false,
        }
    }

    fn show_menu(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Close").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
            })
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_about {
            ctx.show_viewport_immediate(
                ViewportId::from_hash_of("about"),
                ViewportBuilder::default()
                    .with_title("About")
                    .with_inner_size([640.0, 480.0])
                    .with_min_inner_size([640.0, 480.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );
                    about::update_about(ctx, class);
                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_about = false;
                    }
                },
            );
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.show_menu(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cannot open file");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("file:");
                ui.label(&self.current_file.to_string());
            });
            ui.label(format!(
                "missing blender.exe for version \"{}\"",
                &self.version.version
            ));
            ui.add_space(16.0);
            ui.label(format!("{:#?}", &self.version));
        });
    }
}

pub fn open_ui(version: &BlenderVersion, current_file: &str) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let version = Box::new(version.to_owned());
    let current_file = current_file.into();
    eframe::run_native(
        "Open Blender File",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc, version, current_file))),
    )
}
