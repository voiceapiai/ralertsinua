use geo::{BoundingRect, Geometry, Polygon, Rect};
use geojson::de::deserialize_geometry;
use serde::Deserialize;

use crate::utils::*;

pub type WktString = String;
/// x/y bounds as pairs (e.g. for ratatui)
#[allow(non_camel_case_types)]
pub type XY_Bounds = ([f64; 2], [f64; 2]);

const PADDING: f64 = 0.5;

/// Trait for objects that have a bounding rectangle to return x/y bounds as pairs (e.g. for ratatui)
pub trait WithBoundingRect {
    /// Bounding rectangle
    fn bounding_rect(&self) -> Rect;

    /// Bounding rectangle to return x/y bounds as pairs (e.g. for ratatui)
    fn get_x_y_bounds(&self) -> XY_Bounds {
        let rect = self.bounding_rect();
        (
            [rect.min().x - PADDING, rect.max().x + PADDING],
            [rect.min().y - PADDING, rect.max().y + PADDING],
        )
    }
}

/// Ukraine's administrative unit for region - *oblast*
#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    /// OSM Relation Id
    #[serde(rename = "@id")]
    pub relation_id: String,
    /// Alerts.in.ua "uid"
    #[serde(rename = "@location_uid")]
    pub location_uid: i32,
    /// Geometry for borders (Polygon or MultiPolygon)
    #[serde(deserialize_with = "deserialize_geometry")]
    pub geometry: Geometry,
    /// Name in uk
    #[serde(rename = "name")]
    pub name: String,
    /// Name in en
    #[serde(rename = "name:en")]
    pub name_en: String,
}

impl WithBoundingRect for Region {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        self.geometry.bounding_rect().unwrap()
    }
}

impl HasName for Region {
    /// Name in uk
    fn name(&self) -> &str {
        &self.name
    }
}

impl Region {
    /// Geometry for borders (Polygon or MultiPolygon)
    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}

impl Region {
    pub fn borders(&self) -> Option<Polygon> {
        let poly = match &self.geometry {
            Geometry::Polygon(geo) => geo.clone(),
            _ => default_polygon(),
        };

        Some(poly)
    }
}

#[cfg(test)]
mod tests {
    use geo::CoordsIter;

    use super::*;

    #[test]
    fn test_deserialize() {
        use serde_json::json;

        let data = json!({
            "@location_uid":31,
            "@id":"relation/421866",
            "geometry":{"type":"Polygon","coordinates":[[[30.3683029,50.4225715],[30.6435516,50.2260905],[30.6113333,50.3464106],[30.8187002,50.3943757],[30.7376052,50.498925],[30.8158409,50.5639723],[30.719819,50.5908142],[30.5656585,50.5157585],[30.4631088,50.5843452],[30.3072295,50.5704924],[30.2361453,50.4268097],[30.3683029,50.4225715]]]},
            "ISO3166-2":"UA-30","admin_level":"4","boundary":"administrative","int_name":"Kyiv","katotth":"UA80000000000093317","koatuu":"8000000000","name":"Київ","name:de":"Kiew","name:en":"Kyiv","official_name:de":"Kyjiw","place":"city","population":"2908249","source:name:br":"ofis publik ar brezhoneg","timezone":"Europe/Kyiv","type":"boundary","wikidata":"Q1899","wikipedia":"uk:Київ"
        });

        let region = serde_json::from_value(data);
        if region.is_err() {
            let err = region.err().unwrap();
            panic!("Failed to deserialize Region: {:?}", err);
        }
        let region: Region = region.unwrap();

        assert_eq!(region.relation_id, "relation/421866");
        assert_eq!(region.location_uid, 31);
        assert_eq!(region.name(), "Київ");
        assert_eq!(region.name_en, "Kyiv");
        assert_eq!(region.bounding_rect().coords_count(), 4);
        assert_eq!(region.geometry().coords_count(), 12);
    }
}
