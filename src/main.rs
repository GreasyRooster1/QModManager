#![feature(mpmc_channel)]
//#![windows_subsystem = "windows"]

mod log;
mod launch;
mod auth;
mod pack;

use std::env;
use std::ops::Add;
use winresource::WindowsResource;
use std::process::Command;
use std::sync::Mutex;
use eframe::{egui, NativeOptions, WindowBuilderHook};
use eframe::egui::{popup_below_widget, CentralPanel, DragValue, Id, InnerResponse, PopupCloseBehavior, Response, ScrollArea, SidePanel, TopBottomPanel, Ui, IconData, Layout, Align, ProgressBar};
use eframe::egui::Key::P;
use lazy_async_promise::{DataState, DirectCacheAccess, LazyVecPromise, Progress, Promise};
use lazy_static::lazy_static;
use crate::launch::{launch, preform_launch_checks, verify_fml_folder, verify_minecraft_install, LaunchSettings};
use crate::log::{error, info};
use crate::pack::{download_modpack, setup_temp_folder};

const WIDTH:f32  = 1000.;
const HEIGHT:f32  = 700.;
const VERSION:&str = "QModManager - V1.0.1";

#[tokio::main]
async fn main() {
    match setup_temp_folder(){
        Ok(_)=>{}
        Err(_)=>{}
    }
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]).with_icon(load_icon()),
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
    Base,
    Other1,
    Other2,
    Other3,
}

impl Modpack {
    fn get_server_identifier(&self) -> &'static str {
        match self {
            Modpack::ModTeam => "ModTeam",
            Modpack::Base => "Base",
            Modpack::Other1 => "Other1",
            Modpack::Other2 => "Other2",
            Modpack::Other3 => "Other3",
        }
    }
    fn get_name(&self) -> &'static str {
        match self {
            Modpack::ModTeam => "Modded Team Pack",
            Modpack::Base => "Base Pack",
            Modpack::Other1 => "Other 1",
            Modpack::Other2 => "Other 2",
            Modpack::Other3 => "Other 3",
        }
    }
    fn vec_all() -> Vec<Modpack>{
        vec![
            Modpack::ModTeam,
            Modpack::Base,
            Modpack::Other1,
            Modpack::Other2,
            Modpack::Other3,
        ]
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct App {
    update_callback_ctx: Option<egui::Context>,
    download_callback:Option<LazyVecPromise<String>>,

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
            update_callback_ctx: None,
            download_callback: None,
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
    fn update_callback(&self) -> impl Fn() {
        let ctx = self.update_callback_ctx.clone().unwrap();
        move || {  ctx.request_repaint(); }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let ctx_clone = ctx.clone();
        self.update_callback_ctx = Some(ctx_clone);

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
    ui.with_layout(Layout::bottom_up(Align::LEFT),|ui| {
        ui.label(VERSION);
        ui.label(format!("{}",app.debug_console_content));
    });
    let mut new_logs = match &mut app.download_callback {
        Some(callback) => {
            match callback.poll_state() {
                DataState::Updating(_)=> {
                    let logs = callback.get_value();
                    match logs {
                        None => { None }
                        Some(s) => {
                            if s.len() < 1 {
                                None
                            } else {
                                Some(s)
                            }
                        }
                    }
                }
                _ => {None}
            }
        }
        None => {
            None
        }
    };
    match new_logs {
        None => {}
        Some(logs) => {
            app.debug_console_content.push_str(format!("{0}\n",logs.last().unwrap()).as_str());
            match &mut app.download_callback {
                Some(callback) => {
                    let mut val = callback.get_value_mut();
                    val = Some(&mut vec![]);
                }
                _ => {}
            }
        }
    }

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
                for pack in Modpack::vec_all() {
                    ui.selectable_value(&mut app.modpack,pack.clone(), pack.get_name());
                }
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
        match &mut app.download_callback {
            None => {
                if ui.button("LAUNCH").clicked() {
                    info("Launch button clicked", app);
                    let launch_settings = LaunchSettings::from_app(app);
                    launch(app, &launch_settings);
                }
            }
            Some(callback) => {
                let state = callback.poll_state();
                let progress = state.get_progress();
                if let Some(progress) = progress {
                    ui.add(
                        ProgressBar::new(progress.as_f32())
                            .show_percentage()
                            .animate(true)
                    );
                }else{
                    if ui.button("RESET").clicked(){
                        info("resetting back to launch state, this does not remove any mods",app);
                        app.download_callback = None;
                    }
                }
            }
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

pub(crate) fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../Icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}