use std::char;
use std::io::{self, Write};

use color_eyre::eyre::{Context, Result};
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::contains::Contains;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::simplify_vw::SimplifyVw;
use geo::{GeoFloat, Geometry, Line, Point, Polygon, Rect};
use num_traits::FromPrimitive;
use rstar::{self, RTree, RTreeNum, RTreeObject, AABB};

const CELL_ROWS: i32 = 4;
const CELL_COLS: i32 = 2;

#[derive(Debug, Clone, PartialEq)]
pub enum GridGeom<T>
where
    T: GeoFloat + RTreeNum + FromPrimitive,
{
    Point(Point<T>),
    Line(Line<T>),
    Polygon(Polygon<T>),
}

impl<T> GridGeom<T>
where
    T: GeoFloat + RTreeNum + FromPrimitive,
{
    /// Simplify geometries into component pieces for GridGeom
    pub fn vec_from_geom(
        geom: Geometry<T>,
        simplification: T,
        is_area: bool,
    ) -> Vec<GridGeom<T>> {
        match geom {
            Geometry::Point(s) => vec![GridGeom::Point(s); 1],
            Geometry::MultiPoint(s) => s.into_iter().map(GridGeom::Point).collect(),
            Geometry::Line(s) => vec![GridGeom::Line(s); 1],
            Geometry::LineString(s) => s
                .simplify_vw(&simplification)
                .lines()
                .map(GridGeom::Line)
                .collect(),
            Geometry::MultiLineString(s) => s
                .into_iter()
                .map(|ls| ls.simplify_vw(&simplification))
                .flat_map(|ls| ls.lines().collect::<Vec<_>>())
                .map(GridGeom::Line)
                .collect(),
            Geometry::Polygon(s) => {
                if is_area {
                    vec![GridGeom::Polygon(s.simplify_vw(&simplification)); 1]
                } else {
                    s.simplify_vw(&simplification)
                        .exterior()
                        .lines()
                        .map(GridGeom::Line)
                        .collect()
                }
            }
            Geometry::MultiPolygon(s) => {
                if is_area {
                    s.simplify_vw(&simplification)
                        .into_iter()
                        .map(GridGeom::Polygon)
                        .collect()
                } else {
                    s.simplify_vw(&simplification)
                        .into_iter()
                        .flat_map(|p| p.exterior().lines().collect::<Vec<_>>())
                        .map(GridGeom::Line)
                        .collect()
                }
            }
            Geometry::Triangle(s) => vec![GridGeom::Polygon(s.to_polygon()); 1],
            Geometry::Rect(s) => vec![GridGeom::Polygon(s.to_polygon()); 1],
            Geometry::GeometryCollection(s) => s
                .into_iter()
                .flat_map(|g| GridGeom::<T>::vec_from_geom(g, simplification, is_area))
                .collect(),
        }
    }
}

impl<T> RTreeObject for GridGeom<T>
where
    T: GeoFloat + RTreeNum + FromPrimitive,
{
    type Envelope = AABB<[T; 2]>;

    fn envelope(&self) -> Self::Envelope {
        match self {
            GridGeom::Point(pt) => AABB::from_point([pt.x(), pt.y()]),
            GridGeom::Line(line) => {
                let bb = line.bounding_rect();
                AABB::from_corners([bb.min().x, bb.min().y], [bb.max().x, bb.max().y])
            }
            GridGeom::Polygon(poly) => {
                let bb = poly.bounding_rect().unwrap();
                AABB::from_corners([bb.min().x, bb.min().y], [bb.max().x, bb.max().y])
            }
        }
    }
}

/// Convert row/col coordinates to associated Braille hex value
pub fn braille_cell_value(row: i32, col: i32) -> u32 {
    match (row, col) {
        (0, 0) => 0x01,
        (0, 1) => 0x08,
        (1, 0) => 0x02,
        (1, 1) => 0x10,
        (2, 0) => 0x04,
        (2, 1) => 0x20,
        (3, 0) => 0x40,
        (3, 1) => 0x80,
        (_, _) => 0x00,
    }
}

/// Add the Braille offset base to the calculated cell value to generate a char
pub fn braille_char(suffix: u32) -> char {
    let braille_offset = 0x2800;
    char::from_u32(braille_offset + suffix).unwrap()
}

pub struct MapGrid<T>
where
    T: GeoFloat + RTreeNum + FromPrimitive,
{
    rows: i32,
    cols: i32,
    bbox: Rect<T>,
    cell_size: [f64; 2],
    inner_cell_size: [f64; 2],
    rtree: RTree<GridGeom<T>>,
}

impl<T> MapGrid<T>
where
    T: GeoFloat + RTreeNum + FromPrimitive,
{
    pub fn new(width: f64, height: f64, rtree: RTree<GridGeom<T>>) -> MapGrid<T> {
        let envelope = rtree.root().envelope();
        let bbox = Rect::new(envelope.lower(), envelope.upper());
        let box_width = bbox.width().to_f64().unwrap();
        let box_height = bbox.height().to_f64().unwrap();

        let box_aspect_ratio = box_width / box_height;
        let term_aspect_ratio = width / height;

        // Clamp dimensions to aspect ratio of the geometry bbox
        let (cols_f, rows_f) = match (
            term_aspect_ratio > 1.0,
            box_aspect_ratio > 2.0,
            term_aspect_ratio > (box_aspect_ratio * 2.0),
        ) {
            // Multiply or divide by 2.0 to account for columns being more narrow than rows
            (true, true, true) | (true, false, _) => {
                (height * box_aspect_ratio * 2.0, height)
            }
            (true, true, _) | (false, _, _) => (width, (width / box_aspect_ratio) / 2.0),
        };

        let cols = f64::ceil(width) as i32;
        let rows = f64::ceil(height) as i32;

        // Get dimensions of individual cells
        let cell_width = box_width / cols_f;
        let cell_height = box_height / rows_f;

        MapGrid {
            bbox,
            rows,
            cols,
            cell_size: [cell_width, cell_height],
            inner_cell_size: [
                cell_width / f64::from(CELL_COLS),
                cell_height / f64::from(CELL_ROWS),
            ],
            rtree,
        }
    }

    /// Iterate through cells, printing one line at a time
    pub fn print(&self) -> Result<()> {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());

        for r in 0..self.rows {
            let mut row_str = "".to_string();
            for c in 0..self.cols {
                let cell_value = self.query_cell_value(r, c);
                row_str.push_str(&braille_char(cell_value).to_string());
            }
            writeln!(handle, "{}", row_str).context("Error printing line")?;
        }
        Ok(())
    }

    // Get the minimum and maximum points of a cell
    fn min_max_points(
        &self,
        row: i32,
        col: i32,
        start_width: f64,
        start_height: f64,
        cell_size: [f64; 2],
    ) -> (Point<T>, Point<T>) {
        let min_x = start_width + (cell_size[0] * f64::from(col));
        let max_y = start_height - (cell_size[1] * f64::from(row));
        let max_x = T::from_f64(min_x + cell_size[0]).unwrap();
        let min_y = T::from_f64(max_y - cell_size[1]).unwrap();

        let min_pt = Point::new(T::from_f64(min_x).unwrap(), min_y);
        let max_pt = Point::new(max_x, T::from_f64(max_y).unwrap());
        (min_pt, max_pt)
    }

    // Determine whether any geometries within a cell intersect
    fn cell_intersects(&self, min_pt: Point<T>, max_pt: Point<T>) -> bool {
        let envelope =
            AABB::from_corners([min_pt.x(), min_pt.y()], [max_pt.x(), max_pt.y()]);
        let poly_bounds = Polygon::from(Rect::new(min_pt, max_pt));

        self.rtree
            .locate_in_envelope_intersecting(&envelope)
            .skip_while(|g| match g {
                GridGeom::Point(pt) => !poly_bounds.contains(pt),
                GridGeom::Line(line) => !poly_bounds.intersects(line),
                GridGeom::Polygon(poly) => !poly_bounds.intersects(poly),
            })
            .take(1)
            .next()
            .is_some()
    }

    /// For a given Braille 2x4 cell, query which cells have geometries in them
    fn query_cell_value(&self, row: i32, col: i32) -> u32 {
        let mut cell_value = 0x00;

        let bbox_min_x = self.bbox.min().x.to_f64().unwrap();
        let bbox_max_y = self.bbox.max().y.to_f64().unwrap();

        // Return early if there are no geometries in the outer cell
        let (outer_min_pt, outer_max_pt) =
            self.min_max_points(row, col, bbox_min_x, bbox_max_y, self.cell_size);
        if !self.cell_intersects(outer_min_pt, outer_max_pt) {
            return cell_value;
        }

        // Get the start offset dimensions based on the outer row and column
        let start_width = (self.cell_size[0] * f64::from(col)) + bbox_min_x;
        let start_height = bbox_max_y - (self.cell_size[1] * f64::from(row));

        for r in 0..CELL_ROWS {
            for c in 0..CELL_COLS {
                let (min_pt, max_pt) = self.min_max_points(
                    r,
                    c,
                    start_width,
                    start_height,
                    self.inner_cell_size,
                );
                // Add the associated cell value if intersecting lines are found
                if self.cell_intersects(min_pt, max_pt) {
                    cell_value += braille_cell_value(r, c);
                }
            }
        }

        cell_value
    }
}

/* #[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;
    use geo_types::LineString;
    use num_traits::cast::ToPrimitive;

    #[test]
    fn test_vec_from_geom() {
        let poly = Polygon::new(LineString::from(vec![(0., 0.), (1., 1.), (1., 0.)]), vec![]);
        assert_eq!(
            GridGeom::<f64>::vec_from_geom(Geometry::Polygon(poly.clone()), 0.01, false),
            vec![
                GridGeom::Line(Line::<f64>::new((0., 0.), (1., 1.))),
                GridGeom::Line(Line::<f64>::new((1., 1.), (1., 0.))),
                GridGeom::Line(Line::<f64>::new((1., 0.), (0., 0.))),
            ]
        );
        assert_eq!(
            GridGeom::<f64>::vec_from_geom(Geometry::Polygon(poly.clone()), 0.01, true),
            vec![GridGeom::Polygon(poly)]
        );
    }

    #[test]
    fn new_clamps_aspect_ratio() {
        let line = GridGeom::Line(Line::new([0., 0.], [5., 1.]));
        let rtree = RTree::bulk_load(vec![line]);
        let grid = MapGrid::new(4., 4., rtree);
        assert_eq!((grid.cols, grid.rows), (4, 4));
        assert!(approx_eq!(f64, grid.cell_size[0], 1.25));
        assert!(approx_eq!(f64, grid.cell_size[1], 2.5));
    }

    #[test]
    fn query_cell_value_returns_value() {
        let rtree = RTree::bulk_load(vec![
            GridGeom::Line(Line::new([0., 0.], [4., 0.])),
            GridGeom::Point(Point::new(0., 1.)),
        ]);
        let grid = MapGrid::new(4., 4., rtree);
        assert_eq!(grid.query_cell_value(0, 0), 0x36);
    }

    #[test]
    fn min_max_points() {
        let rtree = RTree::bulk_load(vec![
            GridGeom::Line(Line::new([0., 0.], [4., 0.])),
            GridGeom::Point(Point::new(0., 1.)),
        ]);
        let grid = MapGrid::new(4., 4., rtree);
        let (row, col) = (0, 0);
        let start_width =
            (grid.cell_size[0] * f64::from(col)) + grid.bbox.min().x.to_f64().unwrap();
        let start_height =
            grid.bbox.max().y.to_f64().unwrap() - (grid.cell_size[1] * f64::from(row));
        assert_eq!(
            grid.min_max_points(0, 0, start_width, start_height, grid.inner_cell_size),
            (Point::<f64>::new(0., 0.5,), Point::<f64>::new(0.5, 1.))
        );
        assert_eq!(
            grid.min_max_points(4, 1, start_width, start_height, grid.inner_cell_size),
            (Point::<f64>::new(0.5, -1.5), Point::<f64>::new(1.0, -1.))
        );
    }
} */
