use eframe::egui;
use crate::settings::{Human, Settings};

pub struct EWindow {
    settings: Settings,

}
impl EWindow {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings
        }
    }
    pub fn run(mut self){
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(320.0, 240.0)),
            ..Default::default()
        };
        eframe::run_native(
            "My egui App",
            options,
            Box::new(|_cc| Box::new(self)),
        );
    }


}
impl eframe::App for EWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Whom'stve?")
                .selected_text(format!("{:?}", self.settings.human))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.settings.human,  Human::Atoms, "Atoms");
                    ui.selectable_value(&mut self.settings.human,  Human::Oberdiah, "Oberdiah");
                    ui.selectable_value(&mut self.settings.human,  Human::QuickToast, "QuickToast");
                }
                );
        });
    }
}
