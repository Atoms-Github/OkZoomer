use std::fmt::format;
use std::fs::DirEntry;
use std::path::Path;
use std::thread::current;
use egui::{Key, Ui};
use egui_extras::{TableBody, TableBuilder, TableRow};
use egui_extras::Column;
use rand::Rng;
use crate::com::Pat;
use crate::logic::controller::Controller;
use crate::logic::view_dir::ViewDir;
use crate::logic::view_item::ViewItem;


pub static VIEW_COLUMNS: i32 = 5;

pub struct View{
    dirty: bool,
    root_dir: ViewDir,
    view_name: String,
    selected_index: i32,
}
impl Default for View{
    fn default() -> Self{
        let cwd = std::env::current_dir().unwrap();
        Self{
            dirty: true,
            root_dir: ViewDir::new(cwd),
            view_name: rand::thread_rng().gen::<i64>().to_string(),
            selected_index: 0
        }
    }
}
impl View{
    pub fn draw(&mut self, ui: &mut Ui) -> bool{
        let mut interacted = false;
        ui.label("View");
        let mut current_item = 0;
        egui::Grid::new(format!("MyGrid {}", self.view_name)).show(ui, |ui : &mut Ui| {
            for file in self.root_dir.get_files(){
                let filename = match file{
                    ViewItem::File(file) => {
                        file.path.file_name().unwrap().to_str().unwrap()
                    },
                    ViewItem::Dir(dir) => {
                        dir.path.file_name().unwrap().to_str().unwrap()
                    }
                };
                let mut button = egui::widgets::Button::new(filename);
                if self.selected_index == current_item {
                    button = button.fill(egui::Color32::from_rgb(0, 0, 255));
                }
                let button = ui.add(button);


                if button.clicked(){
                    interacted = true;
                    self.selected_index = current_item;
                }
                current_item += 1;
                if current_item % VIEW_COLUMNS == 0{
                    ui.end_row();
                }
            }
        });
        return interacted;
        //ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
    }
    pub fn on_key(&mut self, key: &Key){
        let vertical = match key{
            Key::ArrowUp => -1,
            Key::ArrowDown => 1,
            _ => 0
        };
        let horizontal = match key{
            Key::ArrowLeft => -1,
            Key::ArrowRight => 1,
            _ => 0
        };
        let mut new_index = self.selected_index + vertical * VIEW_COLUMNS + horizontal;
        if new_index < 0{
            new_index = 0;
        }
if new_index >= self.root_dir.get_files().len() as i32{
            new_index = self.root_dir.get_files().len() as i32 - 1;
        }

        self.selected_index = new_index;





    }
    pub fn navigate(&mut self, path: Pat){
        todo!()
    }
}