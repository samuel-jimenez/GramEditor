fn main() {
    if let Ok(bundled) = std::env::var("TEHANU_BUNDLE") {
        println!("cargo:rustc-env=TEHANU_BUNDLE={}", bundled);
    }
}
