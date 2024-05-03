#![allow(unused)]
use delegate::delegate;
use geo::{BoundingRect, Coord, Geometry, LineString, MultiPolygon, Polygon, Rect};
use std::str::FromStr;
use wkt::Wkt;
/// Ukraine borders represented as Polygon in WKT file
pub const UKRAINE_BORDERS_POYGON_WKT: &str = include_str!("../../.data/ukraine.wkt");
/// Ukraine bounding box coords tuple - (min_x, min_y), (max_x, max_y)
///
/// <em>Територія України розташована між 44°23' і 52°25' північної широти та між 22°08' і 40°13' східної довготи</em>
pub const UKRAINE_BBOX: [(f64, f64); 2] = [(22.08, 44.23), (40.13, 52.25)];
/// Ukraine center
///
/// <em>Центр України знаходиться в точці з географічними координатами `49°01'` північної широти і `31°02'` східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.</em>
#[allow(unused)]
pub const UKRAINE_CENTER: (f64, f64) = (49.01, 31.02);

const PADDING: f64 = 0.5;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type XYBounds = ([f64; 2], [f64; 2]);

#[inline]
pub fn from_wkt_to_poly(wkts: &str) -> Result<Polygon> {
    let result: Polygon = Wkt::from_str(wkts)?.try_into()?; // .map_err(|_| GeoError::Unknown)?;

    Ok(result)
}

#[inline]
pub fn from_wkt_to_geom(wkts: &str) -> Result<Geometry> {
    let result: Geometry = Wkt::from_str(wkts)?.try_into()?; // .map_err(|_| GeoError::Unknown)?;

    Ok(result)
}

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
            borders: from_wkt_to_poly(UKRAINE_BORDERS_POYGON_WKT).unwrap(),
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
            let geom: Polygon = from_wkt_to_poly(&wkts.unwrap())?;
            let bbox: Rect = geom.bounding_rect().unwrap();
        };
        Ok((
            [bbox.min().x - padding, bbox.max().x + padding],
            [bbox.min().y - padding, bbox.max().y + padding],
        ))
    }
}
