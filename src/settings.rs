use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings{
    pub human : Human,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Human{
    Oberdiah,
    Atoms,
    QuickToast,
}
impl Default for Settings{
    fn default() -> Self{
        Settings{
            human : Human::Oberdiah,
        }
    }
}
impl Settings{
    pub fn load() -> Self{
        // Return a default, if the file doesn't exist.
        if !Path::new("settings.json").exists(){
            return Self::default();
        }
        // Read the file.
        let mut file = File::open("settings.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // Deserialize the file.
        let settings : Settings = serde_json::from_str(&contents).unwrap();
        settings




    }
    pub fn save(&self){
        let data = serde_json::to_string(&self).unwrap();
        let mut file = std::fs::File::create("settings.json").unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}