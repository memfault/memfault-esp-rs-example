use cmake;

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>> {
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    let cmake_dir = env!("CARGO_MANIFEST_DIR");

    let dst = cmake::build(cmake_dir);

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=memfault-firmware-sdk");
    Ok(())
}
