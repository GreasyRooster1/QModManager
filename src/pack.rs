

use std::error::Error;
use std::fmt::format;
use std::{fs, io};
use std::fs::remove_file;
use std::io::{Cursor, Write};
use std::thread;
use std::path::{Path, PathBuf};
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::{error, info};
use reqwest::blocking;
use zip_extract::ZipExtractError;

const TEMP_PATH:&str = "tmp";
const TEMP_MOD_PATH:&str = "tmp\\mod";

pub fn download_modpack(app:&mut App, modpack: Modpack, minecraft_path: String,launch_settings: &LaunchSettings) -> Result<(),String>{
    info(&format!("begin request for {0}",modpack.get_name()),app);

    let url = format!("http://{0}:{1}/{2}",launch_settings.host_ip,launch_settings.host_port,launch_settings.modpack.get_server_identifier());

    info(&format!("url: {}", url),app);

    match remove_file(Path::new(TEMP_PATH).join("zip.zip")) {
        Ok(_) => {
        }
        Err(_) => {
        }
    };

    let zip_file_path = download_zip(app,url)?;
    let mod_folder_path = Path::new(&minecraft_path).join("mods");

    clear_folder(TEMP_MOD_PATH.to_string())?;
    info("cleared temp folder",app);

    clear_folder(mod_folder_path.to_str().unwrap().to_string())?;
    info("cleared mods folder",app);

    extract_zip(zip_file_path,TEMP_MOD_PATH.to_string())?;
    info("extracted zip",app);

    copy_folder(Path::new(TEMP_MOD_PATH),Path::new(&mod_folder_path)).unwrap();
    info("copied mods into mod folder",app);

    Ok(())
}

fn copy_folder(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()>{
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            continue;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn clear_folder(path:String) -> Result<(),String>{
    match fs::remove_dir_all(&path){
        Ok(_) => {}
        Err(err) => {
            return Err(err.to_string())
        }
    }
    match fs::create_dir(&path){
        Ok(_) => {}
        Err(err) => {
            return Err(err.to_string())
        }
    }
    Ok(())
}

fn download_zip(app:&mut App, url:String) -> Result<String, String> {
    let mut response = match  blocking::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            error("Modpack download failed!",app);
            return Err("Failed to get pack".to_string());
        }
    };
    let bytes = response.bytes().unwrap();
    if std::str::from_utf8(bytes.as_ref()).unwrap() == "PACK NOT FOUND"{
        return Err("Cant find pack!".to_string());
    }
    let zip_file_path = Path::new(TEMP_PATH).join("zip.zip");

    let mut file = fs::OpenOptions::new()
        // .create(true) // To create a new file
        .write(true)
        .create(true)
        // either use the ? operator or unwrap since it returns a Result
        .open(&zip_file_path).unwrap();

    file.write_all(&*bytes).unwrap();
    Ok(zip_file_path.as_path().to_str().unwrap().to_string())
}

fn extract_zip(zip_file_path:String,output_path:String)->Result<(),String>{
    let archive: Vec<u8> = fs::read(zip_file_path).unwrap();
    let target_dir = PathBuf::from(output_path); // Doesn't need to exist

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single folder, that folder's contents would be extracted instead.
    match zip_extract::extract(Cursor::new(archive), &target_dir, true) {
        Ok(_) => {}
        Err(err) => {
            return Err(err.to_string())
        }
    };

    Ok(())
}