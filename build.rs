use std::path::Path;

fn main() {
    cc::Build::new()
        .file("tlse.c")
        .define("TLS_AMALGAMATION", None)
        .define("TLS_REEXPORTABLE", None)
        .define("NO_TLS_LEGACY_SUPPORT", None)
        .define("NO_SSL_COMPATIBLE_INTERFACE", None)
        .cargo_warnings(false)
        .compile("tlse");

    if !Path::new("./src/bindings.rs").exists() {
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("tlse.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("./src/bindings.rs")
            .expect("Couldn't write bindings!")
    }
}
