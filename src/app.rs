use std::{
    collections::BTreeMap,
    env,
    fs::{self, File},
    io,
    process::Command,
};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    folder_name: String,
    smapi_path: Option<String>,
}

impl Instance {
    pub fn new(folder_name: String, smapi_path: Option<String>) -> Instance {
        Instance {
            folder_name,
            smapi_path,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manager {
    pub instances: BTreeMap<String, Instance>,
    pub smapi_path: String,
}

impl Manager {
    pub fn new(smapi_path: String) -> Manager {
        let mut app = Manager {
            smapi_path,
            instances: BTreeMap::new(),
        };
        let instance = Instance::new("Mods".into(), None);
        app.instances.insert("Default".into(), instance);
        app
    }

    pub fn load_config() -> Result<Manager, String> {
        let contents = match fs::read_to_string("./config.json") {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        let manager: Manager = match serde_json::from_str(contents.as_str()) {
            Ok(m) => m,
            Err(e) => return Err(e.to_string()),
        };
        Ok(manager)
    }

    pub fn run(&self, instance: Instance) {
        let path = instance.smapi_path.unwrap_or(self.smapi_path.clone());
        let terminal = env::var("TERMINAL").unwrap_or("konsole".into());

        let mut shell = Command::new(terminal)
            .args([
                "-e",
                "steam-run",
                path.as_str(),
                "--mods-path",
                instance.folder_name.as_str(),
            ])
            .spawn()
            .expect("Failed to spawn shell for smapi");
        let result = shell.wait();
        match result {
            Ok(_) => (),
            Err(e) => println!("{e}"),
        }
    }

    pub fn add_instance(&mut self, instance: Instance, name: String) {
        self.instances.insert(name, instance);
    }

    pub fn save(&self) {
        let f = File::create("./config.json").unwrap();
        to_writer_pretty(f, &self).expect("Failed to write to file");
    }
}

pub enum CurrentScreen {
    Main,
    Add,
    Remove,
    Exit,
}

pub enum CurrentlyAdding {
    Name,
    FolderName,
    SmapiPath,
}

pub struct App {
    pub manager: Manager,
    pub screen: CurrentScreen,
    pub adding: Option<CurrentlyAdding>,
    pub name_input: String,
    pub folder_name_input: String,
    pub smapi_path_input: String,
    pub current_instance: String,
}

impl App {
    pub fn new() -> App {
        let manager = match Manager::load_config() {
            Ok(m) => m,
            Err(_) => {
                println!("Creating config file");
                println!("Enter the path to your smapi installation: ");
                let mut smapi_path: String = String::new();
                io::stdin()
                    .read_line(&mut smapi_path)
                    .expect("Failed to read stdin");
                let path = format!("{}/StardewModdingAPI", smapi_path.replace(['\n', '\r'], ""));
                Manager::new(path)
            }
        };
        App {
            manager,
            screen: CurrentScreen::Main,
            adding: None,
            name_input: String::new(),
            folder_name_input: String::new(),
            smapi_path_input: String::new(),
            current_instance: "Default".into(),
        }
    }

    pub fn save_instance(&mut self) {
        if self.smapi_path_input.is_empty() {
            self.manager.add_instance(
                Instance::new(
                    self.folder_name_input.clone(),
                    Some(self.smapi_path_input.clone()),
                ),
                self.name_input.clone(),
            );
        } else {
            self.manager.add_instance(
                Instance::new(self.folder_name_input.clone(), None),
                self.name_input.clone(),
            );
        };

        self.name_input = String::new();
        self.folder_name_input = String::new();
        self.smapi_path_input = String::new();
    }

    pub fn toggle_adding(&mut self) {
        if let Some(adding_mode) = &self.adding {
            match adding_mode {
                CurrentlyAdding::Name => self.adding = Some(CurrentlyAdding::FolderName),
                CurrentlyAdding::FolderName => self.adding = Some(CurrentlyAdding::SmapiPath),
                CurrentlyAdding::SmapiPath => self.adding = Some(CurrentlyAdding::Name),
            }
        } else {
            self.adding = Some(CurrentlyAdding::Name);
        }
    }

    pub fn select_next(&mut self) {
        let keys: Vec<String> = self.manager.instances.keys().cloned().collect();
        let next_key = match keys.iter().position(|key| key == &self.current_instance) {
            Some(index) => keys.get((index + 1) % keys.len()).cloned(),
            None => keys.first().cloned(),
        };
        self.current_instance = next_key.unwrap();
    }

    pub fn select_prev(&mut self) {
        let keys: Vec<String> = self.manager.instances.keys().cloned().collect();
        let prev_key = match keys.iter().position(|key| key == &self.current_instance) {
            Some(index) => {
                if index == 0 {
                    keys.last().cloned()
                } else {
                    keys.get(index - 1).cloned()
                }
            }
            None => keys.first().cloned(),
        };
        self.current_instance = prev_key.unwrap();
    }
}
