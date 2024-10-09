

use std::fmt::{format};
use std::{fs, io};
use std::error::Error;
use std::fs::remove_file;
use std::io::{Cursor, Write};
use std::thread;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::{error, info};
use reqwest::blocking;
use zip_extract::ZipExtractError;

const TEMP_PATH:&str = "tmp";
const TEMP_MOD_PATH:&str = "tmp\\mod";
pub(crate) const TEMP_DATA_PATH:&str = "tmp\\data.dat";

pub fn download_modpack(app:&mut App, modpack: Modpack, minecraft_path: String,launch_settings: &LaunchSettings) -> Result<(),String>{
    info(&format!("begin request for {0}",modpack.get_name()),app);

    let url = format!("http://{0}:{1}/{2}",launch_settings.host_ip,launch_settings.host_port,launch_settings.modpack.get_server_identifier());

    info(&format!("url: {}", url),app);

    fs::write(Path::new(TEMP_DATA_PATH), format!("{}\n{}",url,minecraft_path)).unwrap();

    let handler = thread::spawn(|| {
        return match download_thread() {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    });
    Ok(())
}

pub fn download_thread() -> Result<(),String>{
    match remove_file(Path::new(TEMP_PATH).join("zip.zip")) {
        Ok(_) => {
        }
        Err(_) => {
        }
    };

    let data_string = match fs::read_to_string(TEMP_DATA_PATH){
        Ok(s) => {s}
        Err(e) => {return Err(e.to_string())}
    };

    let mut lines = data_string.lines();

    let url = match lines.next(){
        Some(url) => url.to_string(),
        None => {return Err("cant get url".to_string())}
    };
    let minecraft_path = match lines.next(){
        Some(minecraft_path) => minecraft_path,
        None => {return Err("cant get minecraft path".to_string())}
    };

    let zip_file_path = download_zip(url)?;
    let mod_folder_path = Path::new(minecraft_path).join("mods");

    clear_folder(TEMP_MOD_PATH.to_string())?;

    clear_folder(mod_folder_path.to_str().unwrap().to_string())?;

    extract_zip(zip_file_path,TEMP_MOD_PATH.to_string())?;

    match copy_folder(Path::new(TEMP_MOD_PATH),Path::new(&mod_folder_path)) {
        Ok(_) => {}
        Err(e) => {
            return Err(e.to_string())
        }
    }

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

fn download_zip(url:String) -> Result<String, String> {
    let mut response = match  blocking::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            return Err("Remote server did not respond".to_string());
        }
    };
    let bytes = response.bytes().unwrap();
    match std::str::from_utf8(bytes.as_ref()) {
        Ok(str) => {
            if str =="PACK NOT FOUND"{
                return Err("Remote server does not have pack data".to_string());
            }
        }
        _ => {}
    }
    let zip_file_path = Path::new(TEMP_PATH).join("zip.zip");

    let mut file = match fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .create(true)
        .open(&zip_file_path) {
        Ok(file) => file,
        Err(_) => {
            return Err("Failed to find temp folder".to_string());
        }
    };

    file.write_all(&*bytes).unwrap();
    Ok(zip_file_path.as_path().to_str().unwrap().to_string())
}

pub(crate) fn setup_temp_folder() -> Result<(), Box<dyn Error>>{
    fs::create_dir(TEMP_PATH)?;
    fs::create_dir(TEMP_MOD_PATH)?;
    Ok(())
}

fn extract_zip(zip_file_path:String, output_path:String) ->Result<(),String>{
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