use std::fs::DirEntry;
use crate::com::Pat;
use crate::logic::view_file::ViewFile;
use crate::logic::view_item::ViewItem;

pub struct ViewDir {
    pub path: Pat,
    files: Vec<ViewItem>,
    dirty: bool
}
impl ViewDir {
    pub fn new(path: Pat) -> Self{
        Self{
            path,
            files: Vec::new(),
            dirty: true
        }
    }
    pub fn get_files(&mut self) -> &Vec<ViewItem>{
        if self.dirty{
            self.files = Vec::new();
            let files = std::fs::read_dir(&self.path).unwrap();
            for file in files{
                let file = file.unwrap();
                let path = file.path();
                let name = path.file_name().unwrap().to_str().unwrap();
                if path.is_dir(){
                    self.files.push(ViewItem::Dir(ViewDir::new(path)));
                }else{
                    self.files.push(ViewItem::File(ViewFile::new(path)));
                }
            }
            self.dirty = false;
        }
        &self.files
    }
}