use std::fmt::format;
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::{error, info};
use reqwest::blocking;

pub fn download_modpack(app:&mut App, modpack: Modpack, minecraft_path: String,launch_settings: &LaunchSettings)->Result<(),String>{
    info(&format!("begin request for {0}",modpack.get_name()),app);

    let url = format!("{0}:{1}/{2}",launch_settings.host_ip,launch_settings.host_port,launch_settings.modpack.get_server_identifier());

    info(&format!("url: {}", url),app);

    let mut response = match  blocking::get("127.0.0.1:7878") {
        Ok(resp) => resp,
        Err(err) => {
            error("Modpack download failed!",app);
            return Err(err.to_string());
        }
    };
    info(response.text().unwrap().as_str(),app);
    Ok(())
}