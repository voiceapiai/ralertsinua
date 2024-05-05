use crate::constants::*;
use crate::utils::*;
use geo::{BoundingRect, Coord, Geometry, Polygon, Rect};

const PADDING: f64 = 0.5;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type XYBounds = ([f64; 2], [f64; 2]);

#[derive(Debug)]
pub struct AlertsInUaMapBounds {
    bounding_box: Rect,
    borders: Polygon,
}

impl Default for AlertsInUaMapBounds {
    #[inline]
    fn default() -> Self {
        Self {
            bounding_box: Rect::new(
                Coord::from((UKRAINE_BBOX[0].0, UKRAINE_BBOX[0].1)),
                Coord::from((UKRAINE_BBOX[1].0, UKRAINE_BBOX[1].1)),
            ),
            borders: from_wkt_into(UKRAINE_BORDERS_POYGON_WKT).unwrap(),
        }
    }
}

pub trait AlertsInUaMap: Sync + Send + std::fmt::Debug {
    fn bounding_box(&self) -> &Rect;
    fn borders(&self) -> &Polygon;
    fn get_x_y_bounds(&self, geom: Option<String>) -> Result<([f64; 2], [f64; 2])>;
}

impl AlertsInUaMap for AlertsInUaMapBounds {
    // delegate! { to self { fn bounding_box() -> &Rect; } }

    #[inline]
    fn bounding_box(&self) -> &Rect {
        &self.bounding_box
    }

    #[inline]
    fn borders(&self) -> &Polygon {
        &self.borders
    }

    #[inline]
    fn get_x_y_bounds(&self, wkts: Option<String>) -> Result<XYBounds> {
        let padding = PADDING;
        let bbox: Rect = self.bounding_box;
        if wkts.is_some() {
            let geom: Geometry = from_wkt_into(&wkts.unwrap())?;
            #[allow(unused)]
            let bbox = geom.bounding_rect().unwrap();
        };
        Ok((
            [bbox.min().x - padding, bbox.max().x + padding],
            [bbox.min().y - padding, bbox.max().y + padding],
        ))
    }
}
