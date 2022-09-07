fn main() {
    cc::Build::new()
        .file("src/test.cpp")
        .cpp(true)
        .compile("testcpp");
    let outdir = std::env::var("OUT_DIR").unwrap();
    // this is needed because:
    //   "error: static library 'testcpp' not found. search paths:\n ... etc"
    let _ = std::fs::copy(
        &format!("{}/libtestcpp.a", outdir),
        &format!("{}/testcpp.lib", outdir),
    );
}
