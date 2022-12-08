#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

pub mod external;
pub mod ui;
pub mod com;
pub mod settings;
pub mod logic;

fn main() {
    let settings = settings::Settings::load();
    let mut window = ui::window::Window::new();
    window.run(settings);
}