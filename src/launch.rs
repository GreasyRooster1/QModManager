use crate::App;
use crate::log::{error, warn};


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

pub fn verify_fml_install() -> bool{
    //todo
    false
}

pub fn verify_minecraft_install() -> bool{
    //todo
    false
}

pub fn preform_launch_checks(app:&mut App){
    if !verify_minecraft_install(){
        error("Minecraft is not installed!",app);
        abort_launch(app,LaunchAbortReason::MinecraftMissing);
        return;
    }
    if !verify_fml_install(){
        error("Forge mod loader is not installed!",app);
        abort_launch(app,LaunchAbortReason::FMLMissing);
        return;
    }


}

pub fn abort_launch(app:&mut App,reason: LaunchAbortReason){
    warn(format!("Launch aborted - {0}",reason.as_str()).as_str(),app)
}