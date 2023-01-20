use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::com::Col;

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
impl Human{
    pub fn get_color(&self) -> Col{
        match self{
            Human::Oberdiah => Col::from_rgb(200, 50, 50),
            Human::Atoms => Col::from_rgb(44, 157, 230),
            Human::QuickToast => Col::from_rgb( 50,255, 50),
        }
    }
}
impl Default for Settings{
    fn default() -> Self{
        Settings{
            human : Human::Oberdiah,
        }
    }
}
impl Settings{
    pub fn draw(&mut self, ui: &mut egui::Ui){

        ui.colored_label(self.human.get_color(), "Whomstove?");
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.human))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.human,  Human::Atoms, "Atoms");
                ui.selectable_value(&mut self.human,  Human::Oberdiah, "Oberdiah");
                ui.selectable_value(&mut self.human,  Human::QuickToast, "QuickToast");
                }
            );

    }
    pub fn load() -> Self {
        // Return a default, if the file doesn't exist.
        if !Path::new("settings.json").exists() {
            return Self::default();
        }
        // Read the file.
        let mut file = File::open("settings.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // Deserialize the file.
        let settings: Settings = serde_json::from_str(&contents).unwrap();
        settings
    }
    pub fn save(&self){
        let data = serde_json::to_string(&self).unwrap();
        let mut file = std::fs::File::create("settings.json").unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}