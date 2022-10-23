// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod util;

use crate::gui::init_gui;

fn main() {
    init_gui();
}
