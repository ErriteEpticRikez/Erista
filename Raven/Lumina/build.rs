extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["C:\\Users\\bizmi\\Documents\\Hackathon DO\\Hackthon DO\\Rust Crates\\Erista\\Raven\\Lumina\\src\\client.proto"],
                                &["C:\\Users\\bizmi\\Documents\\Hackathon DO\\Hackthon DO\\Rust Crates\\Erista\\Raven\\Lumina\\src\\"]).unwrap();
}

