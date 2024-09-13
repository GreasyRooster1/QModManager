use std::fmt::format;
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::info;

pub fn download_modpack(app:&mut App, modpack: Modpack){
    info(&format!("begin download for {0}",modpack.get_name()),app);
}