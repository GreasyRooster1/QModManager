use lazy_async_promise::set_error;
use lazy_async_promise::DataState;
use std::fmt::{format, Debug};
use std::{fs, io};
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::{Cursor, Write};
use std::thread;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::sync::Mutex;
use std::time::Duration;
use fs_extra::file;
use lazy_async_promise::{send_data, set_finished, set_progress, unpack_result, LazyVecPromise, Message, Progress};
use lazy_static::lazy_static;
use rand::Rng;
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::{error, info,CallbackLog};
use reqwest::blocking;
use tokio::sync::mpsc::Sender;

use serde::de::DeserializeOwned;
use zip_extract::ZipExtractError;

const TEMP_PATH:&str = "tmp";
const TEMP_MOD_PATH:&str = "tmp\\mod";
pub(crate) const TEMP_DATA_PATH:&str = "tmp\\data.dat";

pub fn download_modpack(app:&mut App, modpack: Modpack, minecraft_path: String,launch_settings: &LaunchSettings) -> Result<(),String>{
    info(&format!("begin request for {0}",modpack.get_name()),app);

    let url = format!("http://{0}:{1}/{2}",launch_settings.host_ip,launch_settings.host_port,launch_settings.modpack.get_server_identifier());

    info(&format!("url: {}", url),app);

    fs::write(Path::new(TEMP_DATA_PATH), format!("{}\n{}",url,minecraft_path)).unwrap();

    let last_id = match app.prev_log_ids.len()<=0{
        false => {
            app.prev_log_ids[app.prev_log_ids.len() - 1]
        }
        true => {
            0
        }
    };



    app.download_callback = Some(make_request_buffer_slice(TEMP_DATA_PATH,last_id));

    Ok(())
}


fn make_request_buffer_slice(
    file_path: &'static str,
    last_id: u32,
) -> LazyVecPromise<CallbackLog> {
    let updater = move |tx: Sender<Message<CallbackLog>>| async move {

        //get data
        let binding = file::read_to_string(file_path).unwrap();
        let data = binding.split('\n').collect::<Vec<&str>>();
        let url = data[0];
        let mc_path = data[1];

        unpack_result!(clear_folder(format!("{0}/mods",data[1])),tx);

        //get metadata
        let response = unpack_result!(reqwest::get(format!("{url}/metadata")).await, tx);
        let text = unpack_result!(response.text().await,tx);
        let files = text.split("\n").collect::<Vec<&str>>();
        if files.len()==0 {
            send_data!(CallbackLog{
                data: "no mods were found for this pack!".parse().unwrap(),
                id: last_id,
            }, tx);
            set_finished!(tx);
        }

        let total_requests = (files.len()+1) as f64;

        send_data!(CallbackLog{
                data: format!("starting download for {0} files",files.len()),
                id: last_id,
            }, tx);
        set_progress!(
            Progress::from_fraction(1, total_requests),
            tx
        );

        let mut c = 1;
        for file in files {
            if file=="" {
                continue;
            }
            let response = unpack_result!(reqwest::get(format!("{url}/{file}")).await, tx);
            let bytes = unpack_result!(response.bytes().await,tx);
            let mut out_file = unpack_result!(File::create(format!("{mc_path}/mods/{file}")),tx);
            unpack_result!(out_file.write_all(&bytes),tx);
            send_data!(CallbackLog{
                data: format!("downloaded file: {file}"),
                id: last_id+c,
            }, tx);
            set_progress!(
                Progress::from_fraction(c, total_requests),
                tx
            );
            c+=1;
        }
        set_finished!(tx);
    };
    LazyVecPromise::new(updater, 6)
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

pub(crate) fn setup_temp_folder() -> Result<(), Box<dyn Error>>{
    fs::create_dir(TEMP_PATH)?;
    fs::create_dir(TEMP_MOD_PATH)?;
    Ok(())
}
