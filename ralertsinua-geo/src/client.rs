#![allow(clippy::borrow_deref_ref)]
use async_trait::async_trait;
use geo::{BoundingRect, Coord, Geometry, Polygon, Rect};
use geojson::de::{deserialize_feature_collection_to_vec, deserialize_geometry};
use getset::Getters;
use serde::Deserialize;

use crate::{constants::*, utils::*};

pub type WktString = String;
#[allow(non_camel_case_types)]
pub type XY_Bounds = ([f64; 2], [f64; 2]);

const PADDING: f64 = 0.5;

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

#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    #[serde(rename = "@id")]
    pub relation_id: String,
    #[serde(rename = "@a_uid")]
    pub a_id: i32,
    #[serde(deserialize_with = "deserialize_geometry")]
    pub geometry: Geometry,
    #[serde(rename = "name:uk")]
    pub name: String,
    #[serde(rename = "name:en")]
    pub name_en: String,
}

impl WithBoundingRect for Region {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        self.geometry.bounding_rect().unwrap()
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

#[derive(Debug, Clone, Getters)]
pub struct AlertsInUaGeoClient {
    #[getset(get = "pub")]
    bounding_rect: Rect,
    #[getset(get = "pub")]
    borders: Polygon, // &'static str,
    regions: Vec<Region>, // &'static [Region; 27],
}

impl Default for AlertsInUaGeoClient {
    #[inline]
    fn default() -> Self {
        let mut regions: Vec<Region> = deserialize_feature_collection_to_vec::<Region>(
            include_str!("../../assets/ukraine.json").as_bytes(),
        )
        .unwrap();
        regions = sort_by_key_uk(regions, |r| r.name.to_string());
        // use geozero::{geojson::*, ToGeo};
        // let geojson: GeoJson = GeoJson(include_str!("../../assets/ukraine.json"));
        // let mut regions_geo_collection = GeometryCollection::new_from(vec![]);
        // if let Ok(Geometry::GeometryCollection(geo_collection)) = geojson.to_geo() {
        //     regions_geo_collection = geo_collection;
        // }

        Self {
            bounding_rect: Rect::new(
                Coord::from((UKRAINE_BBOX[0].0, UKRAINE_BBOX[0].1)),
                Coord::from((UKRAINE_BBOX[1].0, UKRAINE_BBOX[1].1)),
            ),
            borders: from_wkt_into(UKRAINE_BORDERS_POYGON_WKT).unwrap(),
            regions,
        }
    }
}

impl WithBoundingRect for AlertsInUaGeoClient {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        self.bounding_rect().clone()
    }
}

/// The API for the AlertsInUaClient
#[async_trait]
pub trait AlertsInUaGeo: WithBoundingRect + Sync + Send + core::fmt::Debug {
    fn borders(&self) -> &Polygon;
    fn regions(&self) -> &[Region];

    /* fn get_region_by<P>(&self, mut predicate: P) -> Option<&Region>
    where
        P: FnMut(&Region) -> bool,
    {
        self.regions().iter().find(|r| predicate(r))
    }

    fn get_region_borders_by<P>(&self, predicate: P) -> Option<Polygon>
    where
        P: FnMut(&Region) -> bool,
    {
        self.get_region_by(predicate).unwrap().borders()
    } */
}

#[async_trait]
impl AlertsInUaGeo for AlertsInUaGeoClient {
    #[inline]
    fn borders(&self) -> &Polygon {
        &self.borders
    }

    #[inline]
    fn regions(&self) -> &[Region] {
        self.regions.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use geo::CoordsIter;

    use super::*;

    #[test]
    fn test_default() {
        let geo = AlertsInUaGeoClient::default();
        assert_eq!(geo.borders.coords_count(), 78);
        assert_eq!(geo.regions.len(), 27);
    }
}
