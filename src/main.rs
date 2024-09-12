use eframe::{egui, NativeOptions};

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "QModManager",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    ).unwrap();
}


#[derive(Default)]
struct App {

}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("QModManager");
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Welcome to QModManager")
                });
            });
        });
    }
}

fn create_label<'a>(
    title: &'a str
) -> impl egui::Widget + 'a {
    let label = format!("{title}:");
    move |ui: &mut egui::Ui| {
        ui.label(label)
    }
}
