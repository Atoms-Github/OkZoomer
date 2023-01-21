use crate::com::Pat;

pub struct ViewFile {
    pub path: Pat
}
impl ViewFile {
    pub fn new(path: Pat) -> Self{
        Self{
            path
        }
    }
}