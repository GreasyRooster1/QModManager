use eframe::{egui, NativeOptions, Theme};
use eframe::egui::{CentralPanel, DragValue, InnerResponse, Response, ScrollArea, SidePanel, TopBottomPanel, Ui};

const WIDTH:f32  = 800.;
const HEIGHT:f32  = 400.;

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        default_theme: Theme::Dark,
        ..Default::default()
    };
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
    
    debug_console_content:String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::Minecraft,
            host_ip: "10.0.0.0".to_string(),
            host_port: 255,
            modpack: Modpack::ModTeam,
            debug_console_content: "".to_string(),
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
            TopBottomPanel::top("top_panel")
                .resizable(false)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        top_panel(ui,self);
                    });
                });

            SidePanel::left("left_panel")
                .resizable(true)
                .default_width(250.0)
                .width_range(80.0..=300.0)
                .show_inside(ui, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        left_panel(ui,self);
                    });
                });

            SidePanel::right("right_panel")
                .resizable(true)
                .default_width(250.0)
                .width_range(80.0..=300.0)
                .show_inside(ui, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        right_panel(ui,self);
                    });
                });

            TopBottomPanel::bottom("bottom_panel")
                .resizable(false)
                .min_height(0.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        bottom_panel(ui,self);
                    });
                });

            CentralPanel::default().show_inside(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    center_panel(ui,self);
                });
            });

        });
    }
}

fn top_panel(ui: &mut Ui, app: &mut App){
    create_centered_heading("QModManager",ui);
}

fn left_panel(ui: &mut Ui, app: &mut App){
    ui.vertical(|ui| {
        ui.heading("Game Settings");
        ui.label("change settings about the game");

        line_break(ui);

        egui::ComboBox::from_label("Select Game")
            .selected_text(format!("{0:?}",app.game))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.game, Game::Minecraft, "Minecraft");
            });

    });
}

fn center_panel(ui: &mut Ui, app: &mut App){
    ui.label(format!("{}",app.debug_console_content));
}

fn right_panel(ui: &mut Ui, app: &mut App){
    ui.vertical(|ui| {
        ui.heading("Modpack Settings");
        ui.label("change modpack and server settings");

        line_break(ui);

        ui.label("Host IP:");
        ui.text_edit_singleline(&mut app.host_ip);

        ui.label("Host Port:");
        ui.add(
            DragValue::new(&mut app.host_port)
            .range(0..=65535)
        );

        line_break(ui);

        egui::ComboBox::from_label("Modpack")
            .selected_text(format!("{0:?}",app.modpack.get_name()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.modpack, Modpack::ModTeam, Modpack::ModTeam.get_name());
                ui.selectable_value(&mut app.modpack, Modpack::Base, Modpack::Base.get_name());
            });
    });
}

fn bottom_panel(ui: &mut Ui, app: &mut App){
    ui.vertical_centered(|ui| {
        if ui.button("LAUNCH").clicked() {
            println!("launched")
        }
    });
}

fn line_break(ui: &mut Ui) -> Response {
    ui.label("")
}

fn create_centered_heading(title: &str, ui: &mut Ui) -> InnerResponse<()> {
    ui.vertical_centered(|ui|{
        ui.heading(title);
    })
}
