fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen::EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .emit()?;
    Ok(())
}
