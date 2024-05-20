use eframe::egui;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const LICENSES: &str = include_str!("../../assets/licenses.txt");
const SELF_LICENSE: &str = include_str!("../../LICENSE");

fn show_scroll_area(ui: &mut egui::Ui, lines: &Vec<&str>, total_rows: usize) {
    let row_height = ui.text_style_height(&egui::TextStyle::Body);
    egui::ScrollArea::vertical().auto_shrink(false).show_rows(
        ui,
        row_height,
        total_rows,
        |ui, row_range| {
            for row in row_range {
                ui.label(lines[row]);
            }
        },
    );
}

pub fn update_about(ctx: &egui::Context, _class: egui::ViewportClass) {
    egui::TopBottomPanel::top("self_about")
        .min_height(150.0)
        .show(ctx, |ui| {
            ui.heading(format!("{} {}", PACKAGE_NAME, VERSION));
            let license_rows = SELF_LICENSE.split('\n').collect();
            show_scroll_area(ui, &license_rows, license_rows.len());
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        let licenses_rows = LICENSES.split('\n').collect();
        show_scroll_area(ui, &licenses_rows, licenses_rows.len());
    });
}
