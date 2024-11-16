use lazy_async_promise::DataState;
use std::fmt::{format, Debug};
use std::{fs, io};
use std::error::Error;
use std::fs::remove_file;
use std::io::{Cursor, Write};
use std::thread;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::sync::Mutex;
use std::time::Duration;
use lazy_async_promise::{send_data, set_finished, set_progress, unpack_result, LazyVecPromise, Message, Progress};
use lazy_static::lazy_static;
use crate::{App, Modpack};
use crate::launch::LaunchSettings;
use crate::log::{error, info};
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

    app.download_callback = Some(make_request_buffer_slice("idk"));

    Ok(())
}


fn make_request_buffer_slice<T: DeserializeOwned + Debug + Send + 'static>(
    url: &'static str,
) -> LazyVecPromise<T> {
    let updater = move |tx: Sender<Message<T>>| async move {
        for i in 0..10 {
            //basically a return
            //send_data!(entry, tx);
            set_progress!(
                Progress::from_fraction(i, 10),
                tx
            );
            tokio::time::sleep(Duration::from_millis(100)).await;
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
