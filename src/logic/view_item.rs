use crate::logic::view_dir::ViewDir;
use crate::logic::view_file::ViewFile;

pub enum ViewItem {
    Dir(ViewDir),
    File(ViewFile)
}