#![windows_subsystem = "windows"]

mod log;
mod launch;
mod auth;
mod pack;

use std::process::Command;
use eframe::{egui, NativeOptions, Theme};
use eframe::egui::{popup_below_widget, CentralPanel, DragValue, Id, InnerResponse, PopupCloseBehavior, Response, ScrollArea, SidePanel, TopBottomPanel, Ui};
use crate::launch::{launch, preform_launch_checks, verify_fml_folder, verify_minecraft_install, LaunchSettings};
use crate::log::{error, info};
use crate::pack::{download_modpack, setup_temp_folder};

const WIDTH:f32  = 1000.;
const HEIGHT:f32  = 700.;


fn main() {
    match setup_temp_folder(){
        Ok(_)=>{}
        Err(_)=>{}
    }
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

#[derive(Debug, PartialEq,Clone)]
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

    modpack: Modpack,
    minecraft_version:String,
    forge_version:String,

    is_cracked:bool,

    host_ip:String,
    host_port:i32,

    auth_username:String,
    auth_password:String,
    
    debug_console_content:String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::Minecraft,
            modpack: Modpack::ModTeam,
            minecraft_version: "1.20.1".to_string(),
            forge_version: "47.3.10".to_string(),
            is_cracked: false,
            host_ip: "24.4.89.35".to_string(),
            host_port: 7878,
            auth_username: "".to_string(),
            auth_password: "Mine2021!".to_string(),
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
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
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

        line_break(ui);

        ui.label("Minecraft Version");
        ui.text_edit_singleline(&mut app.minecraft_version);
        ui.label("Forge Version");
        ui.text_edit_singleline(&mut app.forge_version);

        line_break(ui);


        // ui.label("Username");
        // ui.text_edit_singleline(&mut app.auth_username);
        // ui.label("Password");
        // ui.add(
        //     egui::TextEdit::singleline(&mut app.auth_password).password(true).interactive(!app.is_cracked),
        // );
        //
        // ui.checkbox(&mut app.is_cracked,"Is Cracked?")
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

        line_break(ui);

        ui.label("Advanced Options");

        if ui.button("Open Game Folder").clicked(){
            Command::new( "explorer" )
                .arg(verify_minecraft_install().unwrap())
                .spawn( )
                .unwrap( );
        }

        if ui.button("Download Modpack Raw").clicked(){
            match download_modpack(app,app.modpack.clone(),verify_minecraft_install().unwrap(),&LaunchSettings::from_app(app)) {
                Ok(_) => {}
                Err(err) => {
                    error(&format!("Failed to download modpack: {err}"),app)
                }
            }
        }
    });
}

fn bottom_panel(ui: &mut Ui, app: &mut App){
    ui.vertical_centered(|ui| {
        if ui.button("LAUNCH").clicked() {
            info("Launch button clicked",app);
            let launch_settings = LaunchSettings::from_app(app);
            launch(app,&launch_settings);
        }
    });
}

fn line_break(ui: &mut Ui) -> Response {
    ui.label("");
    ui.separator();
    ui.label("")
}

fn create_centered_heading(title: &str, ui: &mut Ui) -> InnerResponse<()> {
    ui.vertical_centered(|ui|{
        ui.heading(title);
    })
}
