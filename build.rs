use std::env;
use std::path::PathBuf;

use bindgen;
use embed_resource;
use winres;

fn main() {
    println!("cargo:rustc-link-search=native=LibreHardwareMonitorAPI\\x64\\Release");
    println!("cargo:rustc-link-lib=dylib=ManagedLibreHardwareMonitorWrapper");

    let bindings = bindgen::Builder::default()
        .header("bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // forces app to run as admin
    embed_resource::compile("program.rc", embed_resource::NONE);

    winres::WindowsResource::new()
        .set_icon("icon.ico")
        .compile().expect("failed to add icon to executable");
}
