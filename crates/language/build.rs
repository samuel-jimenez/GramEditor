fn main() {
    if let Ok(bundled) = std::env::var("GRAM_BUNDLE") {
        println!("cargo:rustc-env=GRAM_BUNDLE={}", bundled);
    }
}
