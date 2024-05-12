use std::str::FromStr;

use geo::{Coord, LineString, Polygon};
use geojson::de::deserialize_feature_collection_str_to_vec;
use icu_locid::subtags::Language;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[inline]
pub fn from_wkt_into<T>(wkts: &str) -> Result<T>
where
    T: TryFrom<wkt::Wkt<f64>>,
    <T as TryFrom<wkt::Wkt<f64>>>::Error: std::error::Error + 'static,
{
    let result: T = wkt::Wkt::from_str(wkts)?.try_into()?;
    Ok(result)
}

pub fn deserialize_feature_collection_to_fixed_array<T, const CAP: usize>(
    geojson_str: &str,
    locale_str: &str,
) -> Result<[T; CAP]>
where
    T: serde::de::DeserializeOwned + Clone + WithName,
{
    let mut features: Vec<T> =
        deserialize_feature_collection_str_to_vec(geojson_str).unwrap();
    features.sort_by_key_icu(|f| f.name().to_string(), locale_str);
    let fixed_array: [T; CAP] = core::array::from_fn(|i| features[i].clone());
    Ok(fixed_array)
}

/// Besides the closed LineString guarantee, the Polygon structure does not enforce validity
///  at this time. For example, it is possible to construct a Polygon that has
/// fewer than 3 coordinates per LineString ring
#[inline]
pub fn default_polygon() -> Polygon {
    Polygon::new(LineString::from(Vec::<Coord>::new()), vec![])
}

pub trait WithName {
    fn name(&self) -> &str;

    fn name_en(&self) -> &str;

    fn get_name_by_locale<L>(&self, locale: L) -> &str
    where
        L: FromStr,
        Language: From<L>,
    {
        let locale: Language = locale.into();
        if locale.as_str() == "uk" {
            return self.name();
        } else {
            return self.name_en();
        };
    }
}

/// Sort a `Vec<T>` (objects) by comparing strings according to language-dependent conventions.
pub trait SortByKeyIcu<T> {
    /// Sort a `Vec<T>` (objects) by comparing strings according to language-dependent conventions.
    fn sort_by_key_icu<F, K>(&mut self, f: F, l: &str)
    where
        F: Fn(&T) -> K,
        K: Into<String> + Sized;
}

impl<T> SortByKeyIcu<T> for Vec<T> {
    #[inline]
    fn sort_by_key_icu<F, K>(&mut self, mut f: F, l: &str)
    where
        F: FnMut(&T) -> K,
        K: Into<String> + Sized,
    {
        use icu_collator::*;
        use icu_locid::Locale;
        // https://github.com/unicode-org/icu4x/tree/main/components/collator#examples
        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Secondary);
        let locale: Locale = l.parse().unwrap();
        let collator: Collator = Collator::try_new(&locale.into(), options).unwrap();

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
    use crate::location::Location;

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
        test_data.sort_by_key_icu(|r| r.name.to_string(), "uk");

        assert_eq!(test_data[0].name, "Вінницька область");
        assert_eq!(test_data[1].name, "Івано-Франківська область");
        assert_eq!(test_data[2].name, "Київ");
        assert_eq!(test_data[3].name, "Київська область");
    }

    #[test]
    fn test_default_polygon() {
        let poly = default_polygon();
        assert_eq!(poly.exterior().coords().count(), 0);
    }

    #[test]
    fn test_deserialize_feature_collection_to_fixed_array() {
        let geojson_str = r#"{
                "type": "FeatureCollection",
                "features": [
                    {
                        "type":"Feature",
                        "id":"relation/421866",
                        "geometry":{"type":"Polygon","coordinates":[[[30.3683029,50.4225715],[30.6435516,50.2260905],[30.6113333,50.3464106],[30.8187002,50.3943757],[30.7376052,50.498925],[30.8158409,50.5639723],[30.719819,50.5908142],[30.5656585,50.5157585],[30.4631088,50.5843452],[30.3072295,50.5704924],[30.2361453,50.4268097],[30.3683029,50.4225715]]]},
                        "properties":{"@location_uid":31,"@id":"relation/421866","ISO3166-2":"UA-30","admin_level":"4","boundary":"administrative","int_name":"Kyiv","katotth":"UA80000000000093317","koatuu":"8000000000","name":"Київ","name:de":"Kiew","name:en":"Kyiv","official_name:de":"Kyjiw","place":"city","population":"2908249","source:name:br":"ofis publik ar brezhoneg","timezone":"Europe/Kyiv","type":"boundary","wikidata":"Q1899","wikipedia":"uk:Київ"}
                    }
                ]
            }"#;
        let locations: [Location; 1] =
            deserialize_feature_collection_to_fixed_array(&geojson_str, "uk").unwrap();
        assert_eq!(locations.len(), 1);

        let location = &locations[0];
        assert_eq!(location.location_uid, 31);
    }
}
