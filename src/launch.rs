use std::path::Path;
use directories::{BaseDirs, ProjectDirs};
use crate::App;
use crate::log::{error, info, warn};


enum LaunchAbortReason{
    MinecraftMissing,
    FMLMissing,
    RuntimeException,
}

impl LaunchAbortReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            LaunchAbortReason::MinecraftMissing => "MinecraftMissing",
            LaunchAbortReason::FMLMissing => "FMLMissing",
            LaunchAbortReason::RuntimeException => "RuntimeException",
        }
    }
}

pub fn verify_fml_install(minecraft_path: &Path) -> bool{
    let fml_path = minecraft_path.join("fml");
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

pub fn preform_launch_checks(app:&mut App){
    let minecraft_path = match verify_minecraft_install(){
        Ok(path) => {
            info(format!("Detected Minecraft @ {0}",path).as_str(), app);
            Path::new(&path)
        }
        Err(_) => {
            error("Minecraft is not installed!", app);
            abort_launch(app, LaunchAbortReason::MinecraftMissing);
            return;
        }
    };


    if !verify_fml_install(minecraft_path){
        error("Forge mod loader is not installed!",app);
        abort_launch(app,LaunchAbortReason::FMLMissing);
        return;
    }


}

pub fn abort_launch(app:&mut App,reason: LaunchAbortReason){
    warn(format!("Launch aborted - {0}",reason.as_str()).as_str(),app)
}