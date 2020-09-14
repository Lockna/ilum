use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_ld = include_bytes!("link.ld");
    let mut f = File::create(out_dir.join("link.ld")).unwrap();
    f.write_all(link_ld).unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-lib=static=ilum-init");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.ld");

}