use std::fmt::format;
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::{error, info};
use reqwest::blocking;

const TEMP_PATH:&str = "tmp";

pub fn download_modpack(app:&mut App, modpack: Modpack, minecraft_path: String,launch_settings: &LaunchSettings)->Result<(),String>{
    info(&format!("begin request for {0}",modpack.get_name()),app);

    let url = format!("http://{0}:{1}/{2}",launch_settings.host_ip,launch_settings.host_port,launch_settings.modpack.get_server_identifier());

    info(&format!("url: {}", url),app);

    let mut response = match  blocking::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            error("Modpack download failed!",app);
            return Err(err.to_string());
        }
    };

    let mod_folder_path = Path::new(&minecraft_path).join("mods");

    let zip_file_path = Path::new(TEMP_PATH).join("zip.zip");

    let mut file = fs::OpenOptions::new()
        // .create(true) // To create a new file
        .write(true)
        .create(true)
        // either use the ? operator or unwrap since it returns a Result
        .open(zip_file_path).unwrap();

    file.write_all(&*response.bytes().unwrap()).unwrap();

    Ok(())
}