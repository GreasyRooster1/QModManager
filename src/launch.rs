use std::fs::{copy, File, OpenOptions};
use std::io::{stdout, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use ansi_term::Color::Red;
use directories::{BaseDirs, ProjectDirs};
use log::info;
use crate::{App, Modpack};
use crate::log::{error, info, warn};
use crate::pack::{download_modpack};

pub struct LaunchSettings{
    pub(crate) forge_version: String,
    pub(crate) minecraft_version: String,

    pub(crate) modpack: Modpack,

    pub(crate) is_cracked:bool,
    pub(crate) auth_username:String,
    pub(crate) auth_password:String,

    pub(crate) host_ip:String,
    pub(crate) host_port:i32,
}

impl LaunchSettings{
    pub fn from_app(app:&App) ->LaunchSettings{
       LaunchSettings{
            forge_version:app.forge_version.clone(),
            minecraft_version:app.minecraft_version.clone(),
            modpack: app.modpack.clone(),
            is_cracked:app.is_cracked.clone(),
            auth_username: app.auth_username.clone(),
            auth_password: app.auth_password.clone(),
            host_ip: app.host_ip.clone(),
            host_port: app.host_port.clone(),
        }
    }
}

enum LaunchAbortReason{
    MinecraftMissing,
    FMLMissing,
    FMLMalformed,
    NoArgumentsFound,
    NetworkError,
    RuntimeException,
}

impl LaunchAbortReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            LaunchAbortReason::MinecraftMissing => "MinecraftMissing",
            LaunchAbortReason::FMLMissing => "FMLMissing",
            LaunchAbortReason::FMLMalformed => "FMLMalformed",
            LaunchAbortReason::NoArgumentsFound => "NoArgumentsFound",
            LaunchAbortReason::NetworkError => "NetworkError",
            LaunchAbortReason::RuntimeException => "RuntimeException",
        }
    }
}

pub fn verify_minecraft_install() -> Result<String, ()> {
    match BaseDirs::new() {
        None => Err(()),
        Some(base_dirs) => {
            let str = format!("{0}\\.minecraft", base_dirs.config_dir().to_str().unwrap());
            let minecraft_path = Path::new(&str);
            match minecraft_path.exists() {
                true => Ok(minecraft_path.to_str().unwrap().to_string()),
                false => Err(())
            }
        }
    }
}

pub fn verify_fml_folder(minecraft_path: &Path, launch_settings: &LaunchSettings) -> Result<String, ()>{
    let forge_handle = format!("{0}-forge-{1}",launch_settings.minecraft_version,launch_settings.forge_version);
    let fml_path = minecraft_path.join("versions").join(forge_handle);
    match fml_path.exists() {
        true => Ok(fml_path.to_str().unwrap().to_string()),
        false => Err(())
    }
}

pub fn verify_fml_installed_correctly(fml_path: &Path, launch_settings: &LaunchSettings) -> Result<String, ()>{
    let jar_file = format!("{0}-forge-{1}.jar", launch_settings.minecraft_version, launch_settings.forge_version);
    match fml_path.join(jar_file.clone()).exists() {
        true => Ok(jar_file),
        false => Err(())
    }
}

pub fn preform_launch_checks(app:&mut App,launch_settings: &LaunchSettings)->Result<(String,String,String),()>{
    let minecraft_path = match verify_minecraft_install(){
        Ok(path) => {
            info(format!("Detected Minecraft @ {0}",path).as_str(), app);
            path
        }
        Err(_) => {
            error("Minecraft is not installed!", app);
            abort_launch(app, LaunchAbortReason::MinecraftMissing);
            return Err(());
        }
    };


    let fml_path = match verify_fml_folder(Path::new(&minecraft_path), &launch_settings){
        Ok(path) => {
            info(format!("Detected FML @ {0}",path).as_str(), app);
            path
        }
        Err(_) => {
            error("FML is not installed!", app);
            abort_launch(app, LaunchAbortReason::FMLMissing);
            return Err(());
        }
    };

    let fml_jar = match verify_fml_installed_correctly(Path::new(&fml_path), &launch_settings){
        Ok(jar) => {
            info(format!("Forge is installed correctly ({0})",jar).as_str(), app);
            jar
        }
        Err(_) => {
            error("You need to launch minecraft to install forge", app);
            abort_launch(app, LaunchAbortReason::FMLMalformed);
            return Err(());
        }
    };

    info("Launch checks passed successfully",app);

    Ok((minecraft_path,fml_path,fml_jar))
}

pub fn launch(app:&mut App,launch_settings: &LaunchSettings){
    match preform_launch_checks(app,launch_settings) {
        Ok((minecraft_path,fml_path,fml_jar)) => {
            match download_modpack(app,launch_settings.modpack.clone(),minecraft_path,launch_settings) {
                Ok(_) => {
                    info("Pack collected successfully",app);
                    launch_client(app,launch_settings);
                }
                Err(err) => {
                    error(format!("Pack did not download correctly -> {err}").as_str(),app);
                    abort_launch(app, LaunchAbortReason::NetworkError);
                    return;
                }
            };


        }
        Err(_) => {
            error("Launch checks failed", app);
            return;
        }
    }
}

fn launch_client(app:&mut App,launch_settings: &LaunchSettings) {
    match Command::new("C:\\Program Files (x86)\\Minecraft Launcher\\MinecraftLauncher.exe").spawn() {
        Ok(_) => {}
        Err(_) => {
            info("Cant launch at default location",app);
            match Command::new("G:\\Minecraft Launcher\\MinecraftLauncher.exe").spawn() {
                Ok(_) => {}
                Err(_) => {
                    info("Cant launch at G Drive location",app);
                    return;
                }
            }
        }
    }
    info("launched successfully",app);
}

pub fn get_launch_command(app:&mut App, fml_path: &Path, fml_jar: &Path, launch_settings: &LaunchSettings) ->Result<String,()>{
    let config_path = fml_path.join(format!("{0}-forge-{1}.json",launch_settings.minecraft_version,launch_settings.forge_version));
    let mut file = match File::open(&config_path){
        Ok(file) => file,
        Err(_) =>{
            error(format!("FML Config not found! ({0})",config_path.display()).as_str(),app);
            return Err(());
        }
    };
    let mut str ="".to_string();
    file.read_to_string(&mut str).unwrap();
    let json:serde_json::Value =  match serde_json::from_str(&*str){
        Ok(json) => json,
        Err(_) => {
            abort_launch(app, LaunchAbortReason::FMLMalformed);
            return Err(());
        }
    };

    let arguments = json.get("arguments").unwrap();
    let jvm = arguments.get("jvm").unwrap().as_array().unwrap();
    let jar = arguments.get("game").unwrap().as_array().unwrap();

    let mut jvm_args = "".to_string();
    for arg in jvm{

        jvm_args.push_str(&arg.to_string().replace("\"",""));
        jvm_args.push_str(" ");
    }
    println!("{:?}", jvm_args);
    let mut jar_args = "".to_string();
    for arg in jar{
        jar_args.push_str(&arg.to_string().replace("\"",""));
        jar_args.push_str(" ");
    }


    Ok(format!("java {jvm_args} -jar {0} {jar_args}",fml_path.join(fml_jar).display()))
}

pub fn abort_launch(app:&mut App,reason: LaunchAbortReason){
    warn(format!("Launch aborted - {0}",reason.as_str()).as_str(),app)
}