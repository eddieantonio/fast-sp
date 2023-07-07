fn main() {
    // compile c library
    let source = "src/count.c";
    cc::Build::new().file(source).compile("libcount.a");
    println!("cargo:rerun-if-changed={source}");
}
