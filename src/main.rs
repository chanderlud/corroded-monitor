// #![windows_subsystem = "windows"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use system::{cpu, gpu, ram};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod ui;
mod system;

// TODO light mode theme is fucked
fn main() { // TODO static linking
    if let Err(error) = ui::main() {
        println!("An error occurred: {:?}", error);
    }
}
