extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn get_crun_dir() -> String {
    let env_path = env::var("CRUN_SRC_DIR");

    if env_path.is_ok() {
        return env_path.unwrap();
    }

    "crunrs_bidings/external/crun".to_owned()
}

fn main() {
    let crun_dir = get_crun_dir();

    // Invoke the shell script to build the external C library
    let status = Command::new("bash")
        .arg("scripts/crun-build")
        .status()
        .expect("Failed to execute crun build script");
    assert!(status.success());

    
    // Specify the path to the generated library
    println!("{}", format!("cargo:rustc-link-search=native={crun_dir}/.libs"));
    println!("cargo:rustc-link-lib=static=crun");
    
    // Generate Rust bindings for the C library
    println!("{}", format!("cargo:rerun-if-changed={crun_dir}/src/crun.h"));
    
    let bindings = bindgen::Builder::default()
        .header(format!("external/crun/src/crun.h"))
        .clang_arg(&crun_dir)
        .clang_arg(format!("-I{crun_dir}/src"))
        .clang_arg(format!("-I{crun_dir}/src/libcrun"))
        .clang_arg(format!("-I{crun_dir}/libocispec"))
        .clang_arg(format!("-I{crun_dir}/libocispec/src"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("crunrs_bidings/src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
