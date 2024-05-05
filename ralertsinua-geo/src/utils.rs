type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[inline]
pub fn from_wkt_into<T>(wkts: &str) -> Result<T>
where
    T: TryFrom<wkt::Wkt<f64>>,
    <T as TryFrom<wkt::Wkt<f64>>>::Error: std::error::Error + 'static,
{
    use std::str::FromStr;
    let result: T = wkt::Wkt::from_str(wkts)?.try_into()?;
    Ok(result)
}
