use std::sync::Arc;

use eframe::egui;

fn main() -> eframe::Result {
    let viewport = egui::ViewportBuilder::default().with_icon(Arc::new(get_icon()));
    let options = eframe::NativeOptions {
        viewport,
        follow_system_theme: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_simple_native("Sorting Algorithm Visualizer", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Sorting Algorithm Visualizer");
                });
                ui.centered_and_justified(|ui| ui.label("Middle"));
            });
        }
    )
}


fn get_icon() -> egui::IconData {
    let icon = include_bytes!("../icon.png");
    let image = image::load_from_memory(icon).unwrap().to_rgba8();

    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.to_vec(),
        width,
        height,
    }
}
