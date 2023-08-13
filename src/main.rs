// disables console for non debug builds only
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use system::{cpu, gpu, ram, storage};

// block non windows builds
#[cfg(not(target_os = "windows"))]
compile_error!("This application only supports Windows.");

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod config;
mod system;
mod ui;

// TODO optional stats, dont render them if they arent available
fn main() {
    if let Err(error) = ui::main() {
        println!("An error occurred: {:?}", error);
    }
}
