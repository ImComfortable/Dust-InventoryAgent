

use eframe::egui;

pub async fn ui() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };


    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        let mut user_input = String::new();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Enter text:");
                ui.text_edit_singleline(&mut user_input);
            });
            if ui.button("Submit").clicked() {
                println!("User input: {}", user_input);
            }
        });
    })
}