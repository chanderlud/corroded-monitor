extern crate embed_resource;

fn main() {
    // forces app to run as admin
    embed_resource::compile("program.rc", embed_resource::NONE);
}

