use eframe::egui;
use crate::logic::controller::Controller;
use crate::logic::view::View;
use crate::settings::{Human, Settings};

pub struct EWindow {
    settings: Settings,
    controller: Controller,
    views: Vec<View>,
}
impl EWindow {
    pub fn new(settings: Settings) -> Self {
        let mut views = Vec::new();
        for i in 0..4 {
            views.push(View::default());
        }
        Self {
            settings,
            controller: Controller::default(),
            views
        }
    }
    pub fn run(mut self){
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(320.0, 240.0)),
            ..Default::default()
        };
        eframe::run_native(
            "Zoomer",
            options,
            Box::new(|_cc| Box::new(self)),
        );
    }
}
impl eframe::App for EWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.settings.draw(ui);
            for view in self.views.iter_mut(){
                view.draw(ui);
            }
        });
    }
}
