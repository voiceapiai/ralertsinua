#![allow(clippy::borrow_deref_ref)]
use geo::{Coord, Polygon, Rect};

use crate::{constants::*, location::*, utils::*};

#[derive(Debug, Clone)]
pub struct AlertsInUaGeoClient {
    pub bounding_rect: Rect,
    pub boundary: Polygon,
    pub locations: [Location; 27],
}

impl Default for AlertsInUaGeoClient {
    #[inline]
    fn default() -> Self {
        let wkt_str = include_str!("../assets/ukraine.wkt");
        let geojson_str = include_str!("../assets/ukraine.json");

        Self {
            bounding_rect: Rect::new(
                Coord::from((UKRAINE_BBOX[0].0, UKRAINE_BBOX[0].1)),
                Coord::from((UKRAINE_BBOX[1].0, UKRAINE_BBOX[1].1)),
            ),
            boundary: from_wkt_into(wkt_str).unwrap(),
            locations: deserialize_feature_collection_to_fixed_array(geojson_str, "uk")
                .unwrap(),
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

    fn get_location_by<P>(&self, mut predicate: P) -> Option<Location>
    where
        P: FnMut(&Location) -> bool,
    {
        self.locations.iter().find(|r| predicate(r)).cloned()
    }
}

/// The API for the AlertsInUaClient
pub trait AlertsInUaGeo: WithBoundingRect + Sync + Send + core::fmt::Debug {
    fn boundary(&self) -> Polygon;
    fn locations(&self) -> [Location; 27];
    fn get_location_by_uid(&self, uid: i32) -> Option<Location>;
    fn get_location_by_name(&self, name: &str) -> Option<Location>;
}

impl AlertsInUaGeo for AlertsInUaGeoClient {
    #[inline]
    fn boundary(&self) -> Polygon {
        self.boundary.clone()
    }

    #[inline]
    fn locations(&self) -> [Location; 27] {
        self.locations.clone()
    }

    #[inline]
    fn get_location_by_uid(&self, location_uid: i32) -> Option<Location> {
        self.get_location_by(|r| r.location_uid == location_uid)
    }

    #[inline]
    fn get_location_by_name(&self, name: &str) -> Option<Location> {
        self.get_location_by(|r| r.name == name)
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
        assert_eq!(geo.bounding_rect.coords_count(), 4);
        assert_eq!(geo.boundary.coords_count(), 955);
        assert_eq!(geo.locations.len(), 27);
    }

    #[test]
    fn test_trait() {
        let geo_client: Arc<dyn AlertsInUaGeo> = Arc::new(AlertsInUaGeoClient::default());
        assert_eq!(geo_client.bounding_rect().coords_count(), 4);
        assert_eq!(geo_client.boundary().coords_count(), 955);
        assert_eq!(geo_client.locations().len(), 27);
    }
}
