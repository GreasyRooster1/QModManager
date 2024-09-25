use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use crate::lib::instance::Instance;
use prettytable::{row, Table};


#[derive(Clone)]
pub struct InstanceManager {
    path: PathBuf,
}

impl InstanceManager {
    pub fn new(pb: PathBuf) -> InstanceManager {
        InstanceManager { path: pb }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn create_instance(&mut self, name: String) -> Option<Instance> {
        let mut ipath = self.path.clone();
        ipath.push(name.clone());

        // TODO: if it exists ask user if they
        //      want to delete the old one and
        //      create a new one

        // create folder if it doesn't exist
        if !ipath.exists() {
            println!("Creating instance dir at {}", ipath.display());
            fs::create_dir(&(*ipath.as_path())).expect("Error creating instances folder");
        } else {
            println!("Directory already exists! Overwriting ...");
        }

        Some(Instance::new(name, ipath))
    }

    // Return paths to all instances invoker files
    pub fn get_list(&mut self) -> Vec<PathBuf> {
        let mut invoker_files: Vec<PathBuf> = Vec::new();

        let instances_dir = self.path.clone();
        for entry in fs::read_dir(instances_dir).unwrap() {
            let entry_path = entry.unwrap().path();
            let mut invoker_file = entry_path.clone();
            invoker_file.push("sml_invoker.json");

            if invoker_file.exists() {
                invoker_files.push(invoker_file);
            }
        }

        invoker_files
    }
}


