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

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Modpack{
    ModTeam,
    Base
}

impl Modpack {
    fn get_server_identifier(&self) -> &'static str {
        match self {
            Modpack::ModTeam => "ModTeam",
            Modpack::Base => "Base",
        }
    }
    fn get_name(&self) -> &'static str {
        match self {
            Modpack::ModTeam => "Modded Team Pack",
            Modpack::Base => "Base Pack",
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct App {
    game: Game,

    host_ip:String,
    host_port:i32,

    modpack: Modpack,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::Minecraft,
            host_ip: "10.0.0.0".to_string(),
            host_port: 255,
            modpack: Modpack::ModTeam,
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

                            line_break(ui);

                            egui::ComboBox::from_label("Select Game")
                            .selected_text(format!("{0:?}",self.game))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.game, Game::Minecraft, "Minecraft");
                            });

                        });
                    });
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Modpack Settings");
                            ui.label("change modpack and server settings");

                            line_break(ui);

                            ui.label("Host IP:");
                            ui.text_edit_singleline(&mut self.host_ip);

                            ui.label("Host Port:");
                            ui.add(egui::DragValue::new(&mut self.host_port).speed(2));

                            line_break(ui);

                            egui::ComboBox::from_label("Modpack")
                                .selected_text(format!("{0:?}",self.modpack.get_name()))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.modpack, Modpack::ModTeam, Modpack::ModTeam.get_name());
                                    ui.selectable_value(&mut self.modpack, Modpack::Base, Modpack::Base.get_name());

                                });
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

fn line_break(ui: &mut Ui) -> Response {
    ui.label("")
}

fn create_centered_heading(title: &str, ui: &mut Ui) -> InnerResponse<()> {
    ui.vertical_centered(|ui|{
        ui.heading(title);
    })
}
