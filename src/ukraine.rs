use geo::{Coord, Rect as BoundingBox};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{Painter, Shape},
};
use serde::*;
use std::fmt::Debug;

// use std::sync::Arc;
// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

/// Ukraine bounding box coords tuple - (min_x, min_y), (max_x, max_y)
///
/// <em>Територія України розташована між 44°23' і 52°25' північної широти та між 22°08' і 40°13' східної довготи</em>
const BOUNDINGBOX: [(f64, f64); 2] = [(22.08, 44.23), (40.13, 52.25)];
/// Ukraine center
///
/// <em>Центр України знаходиться в точці з географічними координатами 49°01' північної широти і 31°02' східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.</em>
#[allow(dead_code)]
const CENTER: (f64, f64) = (49.01, 31.02);

const PADDING: f64 = 0.5;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i32,
    pub geo: String,     // Arc<str>,
    pub name: String,    // Arc<str>,
    pub name_en: String, // Arc<str>,
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
        let boundingbox: BoundingBox = BoundingBox::new(BOUNDINGBOX[0], BOUNDINGBOX[1]);
        Self {
            borders,
            regions,
            color: color.unwrap_or(Color::Yellow),
            boundingbox,
            center: Coord::from(CENTER),
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
    /// Implement the Shape trait for Ukraine to draw map borders on canvas
    #[tracing::instrument(level = "trace")]
    fn draw(&self, painter: &mut Painter) {
        self.borders().for_each(|&coord| {
            if let Some((x, y)) = painter.get_point(coord.x, coord.y) {
                painter.paint(x, y, self.color);
            }
        });
        // TODO: mark center - not working
        if let Some((cx, cy)) = painter.get_point(self.center.x, self.center.y) {
            painter.paint(cx, cy, self.color);
        }
    }
}
