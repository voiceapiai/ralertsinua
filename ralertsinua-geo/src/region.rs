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
    fn bounding_rect(&self) -> Rect;

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
    #[serde(rename = "@a_uid")]
    pub a_id: i32,
    /// Geometry for borders (Polygon or MultiPolygon)
    #[serde(deserialize_with = "deserialize_geometry")]
    pub geometry: Geometry,
    /// Name in uk
    #[serde(rename = "name:uk")]
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
    fn name(&self) -> &str {
        &self.name
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
    // use super::*;

    // #[test]
    // fn test_default() {
    //     let geo = AlertsInUaGeoClient::default();
    //     assert_eq!(geo.borders.coords_count(), 955);
    //     assert_eq!(geo.regions.len(), 27);
    // }

    // #[test]
    // fn test_trait() {
    //     let geo_client: Arc<dyn AlertsInUaGeo> = Arc::new(AlertsInUaGeoClient::default());
    //     assert_eq!(geo_client.borders().coords_count(), 955);
    //     assert_eq!(geo_client.regions().len(), 27);
    // }
}
