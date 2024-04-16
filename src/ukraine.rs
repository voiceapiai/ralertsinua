#[allow(unused)]
use arrayvec::{ArrayString, ArrayVec};
#[allow(unused)]
use either::Either;
use geo::{Coord, Polygon};
use ratatui::{
    layout::Rect,
    prelude::*,
    style::palette::tailwind,
    style::Color,
    widgets::canvas::{Painter, Shape},
    widgets::{ListItem, ListState},
};
use serde::*;
use strum::{Display, EnumString};
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

const REGION_HEADER_BG: Color = tailwind::BLUE.c950;
const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Display)]
pub enum AlertStatus {
    /// Active
    A,
    /// Partially active
    P,
    /// No information
    N,
}

// const A_REGIONS_STATUS_RESPONSE_EXAMPLE: ArrayString<27> = ArrayString::<27>::from("ANNNNNNNNNNNANNNNNNNNNNNNNN");
#[allow(unused)]
const A_REGIONS_IDS: [i8; 27] = [
    29, 4, 8, 9, 28, 10, 11, 12, 13, 31, 14, 15, 16, 27, 17, 18, 19, 5, 30, 20, 21, 22, 23, 3, 24,
    26, 25,
];

pub struct Region_<'a> {
    pub id: i8,
    pub a_id: i8,
    pub osm_id: i64,
    pub geo: &'a str,
    pub name: &'a str,
    pub name_en: &'a str,
    pub status: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i8,
    pub a_id: i8,
    pub osm_id: i64,
    pub geo: String,
    pub name: String,
    pub name_en: String,
    #[sqlx(rename = "status", default)]
    pub status: Option<String>,
}
pub type RegionArrayVec = ArrayVec<Region, 27>;

impl Region {
    #[allow(unused)]
    fn to_list_item(&self, index: usize) -> ListItem {
        let bg_color = match index % 2 {
            0 => NORMAL_ROW_COLOR,
            _ => ALT_ROW_COLOR,
        };
        let line = match self.status.as_deref() {
            Some("N") => Line::styled(format!(" ☐ {}", self.name), TEXT_COLOR),
            Some("A") => Line::styled(
                format!(" ✓ {}", self.name),
                (COMPLETED_TEXT_COLOR, bg_color),
            ),
            _ => todo!(),
        };

        ListItem::new(line).bg(bg_color)
    }
}

#[allow(unused)]
struct StatefulList {
    state: ListState,
    items: RegionArrayVec,
    last_selected: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Ukraine {
    borders: String,
    regions: RegionArrayVec,
    color: Color,
    pub center: Coord,
    pub rect: Rect,
}

impl Ukraine {
    pub fn new(borders: String, regions: RegionArrayVec, color: Option<Color>) -> Self {
        Self {
            borders,
            regions,
            color: color.unwrap_or(Color::Yellow),
            center: Coord::from(CENTER),
            rect: Rect::default(),
        }
    }

    // Iter<'a> = PolygonIter<'a, T>
    pub fn borders(&self) -> Polygon {
        use std::str::FromStr;
        use wkt::Wkt;
        let geom: Polygon = Wkt::from_str(&self.borders).unwrap().try_into().unwrap();
        geom
    }

    pub fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter()
    }

    #[inline]
    pub fn boundingbox(&self) -> [(f64, f64); 2] {
        #[allow(unused_parens)]
        (BOUNDINGBOX)
    }

    #[inline]
    pub fn x_bounds(&self) -> [f64; 2] {
        [
            self.boundingbox().first().unwrap().0 - PADDING,
            self.boundingbox().last().unwrap().0 + PADDING,
        ]
    }

    #[inline]
    pub fn y_bounds(&self) -> [f64; 2] {
        [
            self.boundingbox().first().unwrap().1 - PADDING,
            self.boundingbox().last().unwrap().1 + PADDING,
        ]
    }

    /// Store size of the terminal rect
    #[inline]
    pub fn set_size(&mut self, rect: Rect) {
        self.rect = rect;
    }
    /// Get the resolution of the grid in number of dots
    #[inline]
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
    #[inline]
    fn draw(&self, painter: &mut Painter) {
        let borders = self.borders();
        let coords_iter = borders.exterior().coords().into_iter();
        coords_iter.for_each(|&coord| {
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
