use std::fmt::format;
use std::fs::DirEntry;
use std::path::Path;
use egui::Ui;
use egui_extras::{TableBody, TableBuilder, TableRow};
use egui_extras::Column;
use rand::Rng;
use crate::com::Pat;
use crate::logic::controller::Controller;
use crate::logic::view_dir::ViewDir;
use crate::logic::view_item::ViewItem;

pub struct View{
    dirty: bool,
    root_dir: ViewDir,
    view_name: String,
}
impl Default for View{
    fn default() -> Self{
        let cwd = std::env::current_dir().unwrap();
        Self{
            dirty: true,
            root_dir: ViewDir::new(cwd),
            view_name: rand::thread_rng().gen::<i64>().to_string()
        }
    }
}
impl View{
    pub fn draw(&mut self, ui: &mut Ui){
        ui.label("View");
        let items_per_row = 5;
        let mut current_indent = 0;
        egui::Grid::new(format!("MyGrid {}", self.view_name)).show(ui, |ui : &mut Ui| {
            for file in self.root_dir.get_files(){
                match file{
                    ViewItem::File(file) => {
                        ui.label(file.path.file_name().unwrap().to_str().unwrap());
                    },
                    ViewItem::Dir(dir) => {
                        ui.label(dir.path.file_name().unwrap().to_str().unwrap());
                    }
                }
                current_indent += 1;
                if current_indent >= items_per_row{
                    current_indent = 0;
                    ui.end_row();
                }
            }



        });

        //ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
    }

    pub fn navigate(&mut self, path: Pat){
        todo!()
    }

}