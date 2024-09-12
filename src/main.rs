use eframe::{egui, NativeOptions};
use eframe::egui::{InnerResponse, Response, Ui};

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "QModManager",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    ).unwrap();
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Game{
    Minecraft
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct App {
    game: Game
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::Minecraft,
        }
    }
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
            create_centered_heading("QModManager",ui);
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Game Settings");
                            ui.label("change settings about the game");

                            ui.label("");

                            egui::ComboBox::from_label("Select Game")
                            .selected_text(format!("{0:?}",self.game))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.game, Game::Minecraft, "Minecraft");
                            });

                        });
                    });
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Launch Settings");
                            ui.label("change launcher/modpack settings");
                            ui.label("")
                        });
                    });
                });
                ui.group(|ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button("LAUNCH").clicked() {
                            println!("launched")
                        }
                    });
                });
            });

        });
    }
}



fn create_centered_heading(title: &str, ui: &mut Ui) -> InnerResponse<()> {
    ui.vertical_centered(|ui|{
        ui.heading(title);
    })
}
