use geo::{Coord, LineString, Polygon};

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

#[inline]
pub fn default_polygon() -> Polygon {
    Polygon::new(LineString::from(Vec::<Coord>::new()), vec![])
}

/// Sort a Vector of objects alphabetically, taking into account the locale of a struct property.
pub fn sort_by_key_uk<'a, T, F, K>(list: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(&T) -> K + 'a,
    K: Into<String> + 'a,
{
    use icu::collator::*;
    use icu::locid::{locale, Locale};
    // https://github.com/unicode-org/icu4x/tree/main/components/collator#examples
    let locale: Locale = locale!("uk");
    let mut options = CollatorOptions::new();
    options.strength = Some(Strength::Secondary);
    let collator: Collator = Collator::try_new(&locale.into(), options).unwrap();

    let mut newly_sorted_list = list;
    newly_sorted_list.sort_by(|a: &T, b: &T| {
        let value_a: String = f(a).into();
        let value_b: String = f(b).into();
        collator.compare(&value_a, &value_b)
    });
    newly_sorted_list
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

        let test_data = vec![
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

        let sorted_data = sort_by_key_uk(test_data, |r| r.name.to_string());
        assert_eq!(sorted_data[0].name, "Вінницька область");
        assert_eq!(sorted_data[1].name, "Івано-Франківська область");
        assert_eq!(sorted_data[2].name, "Київ");
        assert_eq!(sorted_data[3].name, "Київська область");
    }
}
