fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if we're inside an Alpine Docker container
    if std::env::var("ALPINE_BUILD").is_ok() {
        println!("cargo:rustc-cfg=feature=\"reqwest-rustls-tls\"");
    }
    Ok(())
}
