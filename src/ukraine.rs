use geo::{Coord, Rect as BoundingBox};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{Painter, Shape},
};
use serde::*;
use std::fmt::Debug;
// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

/// Ukraine bounding box as 2 longitude and 2 latitude
/// [44.18, 52.25, 22.08, 40.13]
/// Територія України розташована між 44° 23' і 52° 25' північної широти та між 22° 08' і 40° 13' східної довготи
const BOUNDINGBOX: [f64; 4] = [44.18, 52.25, 22.08, 40.13];
/// Ukraine center Центр України знаходиться в точці з географічними координатами 49° 01' північної широти і 31° 02' східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.
const CENTER: [f64; 2] = [49.01, 31.02];

const PADDING: f64 = 0.5;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    // pub geo: Geometry,
}

#[derive(Debug)]
pub struct Ukraine {
    borders: Vec<Coord>,
    regions: Vec<Region>,
    color: Color,
    pub boundingbox: BoundingBox,
    pub center: Coord,
    pub rect: Rect,
}

impl Ukraine {
    pub fn new(borders: Vec<Coord>, regions: Vec<Region>, color: Option<Color>) -> Self {
        let boundingbox: BoundingBox = BoundingBox::new((22.08, 44.23), (40.13, 52.25));
        Self {
            borders,
            regions,
            color: color.unwrap_or(Color::Yellow),
            boundingbox,
            center: boundingbox.center(),
            rect: Rect::default(),
        }
    }

    pub fn borders(&self) -> impl Iterator<Item = &Coord> {
        self.borders.iter()
    }

    pub fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter()
    }

    pub fn x_bounds(&self) -> [f64; 2] {
        [
            self.boundingbox.min().x - PADDING,
            self.boundingbox.max().x + PADDING,
        ]
    }
    pub fn y_bounds(&self) -> [f64; 2] {
        [
            self.boundingbox.min().y - PADDING,
            self.boundingbox.max().y + PADDING,
        ]
    }

    /// Store size of the terminal rect
    pub fn set_size(&mut self, rect: Rect) {
        self.rect = rect;
    }
    /// Get the resolution of the grid in number of dots. This doesn't have to be the same as the
    /// number of rows and columns of the grid. For example, a grid of Braille patterns will have a
    /// resolution of 2x4 dots per cell. This means that a grid of 10x10 cells will have a
    /// resolution of 20x40 dots
    pub fn resolution(&self) -> (f64, f64) {
        (
            f64::from(self.rect.width) * 2.0,
            f64::from(self.rect.height) * 4.0,
        )
    }
}

impl Shape for Ukraine {
    fn draw(&self, painter: &mut Painter) {
        self.borders().for_each(|&coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, self.color);
            }
        })
    }
}
