fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen::EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .emit()?;

    toggle_feature_tls();

    Ok(())
}

/// Check if we're inside an Alpine Docker container and enable the Rustls TLS feature if so.
///
/// Environment variables starting with CARGO_FEATURE_ are used by Cargo, the Rust package manager, to control optional features of a crate.
/// When you specify features in the [features] section of your Cargo.toml, Cargo creates environment variables for each feature in the form of CARGO_FEATURE_<FEATURE_NAME>. These environment variables are set to 1 if the feature is enabled and are not set if the feature is disabled.
fn toggle_feature_tls() {
    if std::env::var("ALPINE_BUILD").is_ok() {
        std::env::set_var("CARGO_FEATURE_RUSTLS_TLS", "1");
        println!("cargo:rustc-cfg-features=reqwest-rustls-tls");
    }
}
