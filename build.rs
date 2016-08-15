extern crate serde_codegen;
extern crate glob;

use std::env;
use std::path::Path;
use glob::glob;

fn compile(path: &Path, out_dir: &Path) {
    let dst = Path::new(out_dir.as_os_str()).join(path.file_stem().unwrap());
    serde_codegen::expand(path, &dst).unwrap();
}

pub fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dst_dir = Path::new(&out_dir);
    for entry in glob("src/**/*.*.in").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => compile(path.as_path(), dst_dir),
            Err(e) => println!("{:?}", e),
        }
    }
}