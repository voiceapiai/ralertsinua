#![allow(clippy::borrow_deref_ref)]
use geo::{Coord, Polygon, Rect};

use crate::{constants::*, region::*, utils::*};

#[derive(Debug, Clone)]
pub struct AlertsInUaGeoClient {
    pub bounding_rect: Rect,
    pub borders: Polygon,
    pub regions: [Region; 27],
}

impl Default for AlertsInUaGeoClient {
    #[inline]
    fn default() -> Self {
        let geojson_str = include_str!("../../assets/ukraine.json");

        Self {
            bounding_rect: Rect::new(
                Coord::from((UKRAINE_BBOX[0].0, UKRAINE_BBOX[0].1)),
                Coord::from((UKRAINE_BBOX[1].0, UKRAINE_BBOX[1].1)),
            ),
            borders: from_wkt_into(UKRAINE_BORDERS_POYGON_WKT).unwrap(),
            regions: deserialize_feature_collection_to_fixed_array(geojson_str).unwrap(),
        }
    }
}

impl WithBoundingRect for AlertsInUaGeoClient {
    #[inline]
    fn bounding_rect(&self) -> Rect {
        self.bounding_rect
    }
}

impl AlertsInUaGeoClient {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_region_by<P>(&self, mut predicate: P) -> Option<&Region>
    where
        P: FnMut(&Region) -> bool,
    {
        self.regions.iter().find(|r| predicate(r))
    }
}

/// The API for the AlertsInUaClient
pub trait AlertsInUaGeo: WithBoundingRect + Sync + Send + core::fmt::Debug {
    fn borders(&self) -> &Polygon;
    fn regions(&self) -> &[Region];
    fn get_region_by_uid(&self, uid: usize) -> Option<&Region>;
    fn get_region_by_name(&self, name: &str) -> Option<&Region>;
}

impl AlertsInUaGeo for AlertsInUaGeoClient {
    #[inline]
    fn borders(&self) -> &Polygon {
        &self.borders
    }

    #[inline]
    fn regions(&self) -> &[Region] {
        self.regions.as_slice()
    }

    #[inline]
    fn get_region_by_uid(&self, uid: usize) -> Option<&Region> {
        self.get_region_by(|r| r.a_id == uid as i32)
    }

    #[inline]
    fn get_region_by_name(&self, name: &str) -> Option<&Region> {
        self.get_region_by(|r| r.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::CoordsIter;
    use std::sync::Arc;

    #[test]
    fn test_default() {
        let geo = AlertsInUaGeoClient::default();
        assert_eq!(geo.borders.coords_count(), 955);
        assert_eq!(geo.regions.len(), 27);
    }

    #[test]
    fn test_trait() {
        let geo_client: Arc<dyn AlertsInUaGeo> = Arc::new(AlertsInUaGeoClient::default());
        assert_eq!(geo_client.borders().coords_count(), 955);
        assert_eq!(geo_client.regions().len(), 27);
    }
}
