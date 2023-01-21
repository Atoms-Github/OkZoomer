use eframe::egui;
use egui::{Event, Key};
use crate::logic::controller::Controller;
use crate::logic::view::View;
use crate::settings::{Human, Settings};

pub struct EWindow {
    settings: Settings,
    controller: Controller,
    views: Vec<View>,
    focused_view: usize,

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
            views,
            focused_view: 0
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
            for (i, view) in self.views.iter_mut().enumerate(){
                if view.draw(ui){
                    self.focused_view = i;
                }
            }
            for event in &ui.input().events{
                match event {
                    Event::Key{key, pressed, modifiers} => {
                        if *pressed{
                            self.views[self.focused_view].on_key(key);
                            let new_focus = match &key{
                                Key::R => 0,
                                Key::S => 1,
                                Key::T => 2,
                                Key::N => 3,
                                _ => {500}
                            };
                            if new_focus < 4{
                                self.focused_view = new_focus;
                            }

                        }
                    },
                    _ => {}
                }
            }

        });
    }
}
