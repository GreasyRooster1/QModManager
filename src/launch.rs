use crate::App;
use crate::log::warn;

pub fn check_fml_is_installed(app:&mut App){
    warn("Forge Mod Loader is not currently installed!",app);
}