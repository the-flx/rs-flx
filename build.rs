#[cfg(feature = "bindgen")]
extern crate bindgen;

fn main() {
    // Compile C code.
    cc::Build::new()
        .file("./c_lib/flx-c/src/flx.c")
        .compile("flx");

    // Required for downstream crates.
    println!("cargo:rustc-link-lib=static=flx");

    #[cfg(feature = "bindgen")]
    {
        use std::env;
        use std::path::PathBuf;

        let bindings = bindgen::Builder::default()
            .header("./c_lib/flx-c/include/stb_ds.h")
            .header("./c_lib/flx-c/include/flx.h")
            .prepend_enum_name(false)
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
