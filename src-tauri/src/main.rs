// Sembunyikan jendela console di Windows pada build release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    pomo_lib::run()
}
