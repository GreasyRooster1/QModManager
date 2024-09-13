use std::path::Path;
use directories::{BaseDirs, ProjectDirs};
use crate::App;
use crate::log::{error, info, warn};

pub struct LaunchSettings{
    pub(crate) forge_version: String,
    pub(crate) minecraft_version: String,
}



enum LaunchAbortReason{
    MinecraftMissing,
    FMLMissing,
    FMLMalformed,
    RuntimeException,
}

impl LaunchAbortReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            LaunchAbortReason::MinecraftMissing => "MinecraftMissing",
            LaunchAbortReason::FMLMissing => "FMLMissing",
            LaunchAbortReason::FMLMalformed => "FMLMalformed",
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

pub fn preform_launch_checks(app:&mut App,launch_settings: LaunchSettings){
    let minecraft_path = match verify_minecraft_install(){
        Ok(path) => {
            info(format!("Detected Minecraft @ {0}",path).as_str(), app);
            path
        }
        Err(_) => {
            error("Minecraft is not installed!", app);
            abort_launch(app, LaunchAbortReason::MinecraftMissing);
            return;
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
            return;
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
            return;
        }
    };


}

pub fn abort_launch(app:&mut App,reason: LaunchAbortReason){
    warn(format!("Launch aborted - {0}",reason.as_str()).as_str(),app)
}