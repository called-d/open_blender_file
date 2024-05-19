use eframe::egui;

use crate::version_checker::BlenderVersion;

struct MyApp {
    version: Box<BlenderVersion>,
    current_file: Box<str>,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, version: Box<BlenderVersion>, current_file: Box<str>) -> Self {
        Self {
            version,
            current_file,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cannot open file");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("file:");
                ui.label(&self.current_file.to_string());
            });
            ui.label(format!("missing blender.exe for version \"{}\"", &self.version.version));
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
        Box::new(|cc| {
            Box::new(MyApp::new(cc, version, current_file))
        }),
    )
}
