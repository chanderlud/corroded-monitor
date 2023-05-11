#![windows_subsystem = "windows"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use system::{cpu, gpu, ram, storage};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod ui;
mod system;
mod config;

// TODO optional stats, dont render them if they arent available
// TODO try to setup auto into theme for styles
// TODO scrollable is crashing shit i think (github issue)
fn main() {
    if let Err(error) = ui::main() {
        println!("An error occurred: {:?}", error);
    }
}
