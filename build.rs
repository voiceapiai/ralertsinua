fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen::EmitBuilder::builder()
        .all_build()
        .all_git()
        .emit()?;
    #[allow(deprecated)]
    for item in dotenv::dotenv_iter().unwrap() {
        let (key, val) = item.unwrap();
        println!("cargo:rustc-env={}={}", key, val);
    }
    Ok(())
}
