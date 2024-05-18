use geo::{BoundingRect, Geometry, Polygon, Rect};
use geojson::de::deserialize_geometry;
#[cfg(feature = "tui")]
use ratatui::{
    prelude::*,
    widgets::canvas::{Painter, Shape},
};
use serde::{Deserialize, Serialize};

use crate::utils::*;

/// WKT string
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

/// Ukraine's administrative unit lv4  - *oblast*
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Location {
    /// OSM Relation Id
    #[serde(rename = "@id")]
    pub relation_id: String,
    /// Alerts.in.ua "uid"
    #[serde(rename = "@location_uid")]
    pub location_uid: i32,
    /// "state" or "city" or "special"
    #[serde(rename = "place")]
    pub location_type: String,
    /// Geometry for boundary (Polygon or MultiPolygon)
    #[serde(deserialize_with = "deserialize_geometry")]
    pub geometry: Geometry,
    /// Name in uk
    #[serde(rename = "name")]
    pub name: String,
    /// Name in en
    #[serde(rename = "name:en")]
    pub name_en: String,
}

impl WithBoundingRect for Location {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        self.geometry.bounding_rect().unwrap()
    }
}

impl WithName for Location {
    /// Name in uk
    fn name(&self) -> &str {
        &self.name
    }

    /// Name in en
    fn name_en(&self) -> &str {
        &self.name_en
    }
}

impl Location {
    /// Geometry for boundary (Polygon or MultiPolygon)
    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            relation_id: String::default(),
            location_uid: 0,
            location_type: String::default(),
            geometry: default_polygon().into(),
            name: String::default(),
            name_en: String::default(),
        }
    }
}

impl Location {
    #[inline]
    pub fn boundary(&self) -> &Polygon {
        // If you're sure that the underlying geometry is always a Polygon, you can use the if let construct to try to downcast it. This will allow you to avoid having to provide match arms for all the other possible variants. However, please note that if the underlying geometry is not a Polygon, this will panic at runtime.
        if let Geometry::Polygon(polygon) = &self.geometry {
            polygon
        } else {
            panic!("Expected boundary as Polygon");
        }
    }

    #[inline]
    pub fn center(&self) -> (f64, f64) {
        let rect = self.bounding_rect();
        rect.center().x_y()
    }
}

/// Draws location boundary with [`Canvas`]
#[cfg(feature = "tui")]
impl Shape for Location {
    /// This method draws points of `Polygon` of the location with `Painter` object. It iterates over the exterior coordinates of the boundary and paints each point with `Painter` using the `paint` method
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        self.boundary().exterior().coords().for_each(|coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, Color::Reset);
            }
        });
    }
}

/// Country boundary (borders) as a Polygon
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct CountryBoundary(pub Polygon);

impl Default for CountryBoundary {
    fn default() -> Self {
        Self(default_polygon())
    }
}

/// Draws country boundary with [`Canvas`]
#[cfg(feature = "tui")]
impl Shape for CountryBoundary {
    #[inline]
    /// This method draws points of `Polygon` of the boundary with `Painter` object. It iterates over the exterior coordinates of the boundary and paints each point with `Painter` using the `paint` method
    fn draw(&self, painter: &mut Painter) {
        self.0.exterior().coords().for_each(|coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, Color::Reset);
            }
        });
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
            "ISO3166-2":"UA-30","admin_level":"4","boundary":"administrative","int_name":"Kyiv","name":"Київ","name:de":"Kiew","name:en":"Kyiv","official_name:de":"Kyjiw","place":"city","population":"2908249","timezone":"Europe/Kyiv","type":"boundary","wikidata":"Q1899","wikipedia":"uk:Київ"
        });

        let location = serde_json::from_value(data);
        if location.is_err() {
            let err = location.err().unwrap();
            panic!("Failed to deserialize location: {:?}", err);
        }
        let location: Location = location.unwrap();

        assert_eq!(location.relation_id, "relation/421866");
        assert_eq!(location.location_uid, 31);
        assert_eq!(location.name(), "Київ");
        assert_eq!(location.name_en, "Kyiv");
        assert_eq!(location.location_type, "city");
        assert_eq!(location.bounding_rect().coords_count(), 4);
        assert_eq!(location.geometry().coords_count(), 12);
    }
}
