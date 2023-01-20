use egui::Ui;

pub struct View{

}
impl View{
    pub fn draw(&self, ui: &mut Ui){
        ui.label("View");
    }
}