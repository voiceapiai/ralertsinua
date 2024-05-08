use geo::{Coord, LineString, Polygon};
use geojson::de::deserialize_feature_collection_str_to_vec;
use icu::locid::{locale, Locale};

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

pub fn deserialize_feature_collection_to_fixed_array<T, const CAP: usize>(
    geojson_str: &str,
) -> Result<[T; CAP]>
where
    T: serde::de::DeserializeOwned + Clone + HasName,
{
    let mut features: Vec<T> =
        deserialize_feature_collection_str_to_vec(geojson_str).unwrap();
    assert!(
        features.len() == 27,
        "Geo: Regions count is not equal to 27"
    );
    features.sort_by_key_icu(|f| f.name().to_string(), locale!("uk"));
    let fixed_array: [T; CAP] = core::array::from_fn(|i| features[i].clone());
    Ok(fixed_array)
}

#[inline]
pub fn default_polygon() -> Polygon {
    Polygon::new(LineString::from(Vec::<Coord>::new()), vec![])
}

pub trait HasName {
    fn name(&self) -> &str;
}

pub trait SortByKeyIcu<T> {
    /// Sort a `Vec<T>` (objects) by comparing strings according to language-dependent conventions.
    fn sort_by_key_icu<F, K>(&mut self, f: F, l: Locale)
    where
        F: Fn(&T) -> K,
        K: Into<String> + Sized;
}

impl<T> SortByKeyIcu<T> for Vec<T> {
    fn sort_by_key_icu<F, K>(&mut self, mut f: F, l: Locale)
    where
        F: FnMut(&T) -> K,
        K: Into<String> + Sized,
    {
        use icu::collator::*;
        // https://github.com/unicode-org/icu4x/tree/main/components/collator#examples
        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Secondary);
        let collator: Collator = Collator::try_new(&l.into(), options).unwrap();

        self.sort_by(|a: &T, b: &T| {
            let value_a: String = f(a).into();
            let value_b: String = f(b).into();
            collator.compare(&value_a, &value_b)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_key_uk() {
        #[derive(Debug, Clone)]
        struct TestStruct {
            name: String,
        }

        let mut test_data = vec![
            TestStruct {
                name: "Івано-Франківська область".to_string(),
            },
            TestStruct {
                name: "Київська область".to_string(),
            },
            TestStruct {
                name: "Вінницька область".to_string(),
            },
            TestStruct {
                name: "Київ".to_string(),
            },
        ];
        test_data.sort_by_key_icu(|r| r.name.to_string(), locale!("uk"));

        assert_eq!(test_data[0].name, "Вінницька область");
        assert_eq!(test_data[1].name, "Івано-Франківська область");
        assert_eq!(test_data[2].name, "Київ");
        assert_eq!(test_data[3].name, "Київська область");
    }
}
