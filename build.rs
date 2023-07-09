use std::env;
use std::process::Command;

fn main() {
    compile_c_library();
    generate_test_data();
}

fn compile_c_library() {
    let source = "src/count.c";
    cc::Build::new().file(source).compile("libcount.a");
    println!("cargo:rerun-if-changed={source}");
}

fn generate_test_data() {
    let script_name = "./generate-test-data.py";
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("python3")
        .args([script_name, "-C", &out_dir])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed={script_name}");
}
