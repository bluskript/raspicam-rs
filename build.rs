use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // CFG.include_prefix = "/"
    // let opencv2 = pkg_config::probe_library("opencv4").unwrap();
    // let opencv_include_paths = opencv2.include_paths.iter().map(PathBuf::as_path);
    let mut b = autocxx_build::Builder::new(
        "src/binding_generator.rs",
        &[&PathBuf::from("src"), &PathBuf::from("raspicam/src")],
    )
    .extra_clang_args(&["-std=c++17"])
    .build()?;
    b.flag_if_supported("-std=c++17")
        .includes(vec!["/usr/local/include"])
        .compile("raspicam-rs");
    println!("cargo:rerun-if-changed=src/bindings-generator.rs");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=raspicam");
    Ok(())
}
