use geo::{Coord, Point, Rect};
use lazy_static::lazy_static;

lazy_static! {
    /// Ukraine bounding box coords tuple - (min_x, min_y), (max_x, max_y)
    ///
    /// Територія України розташована між 44°23' і 52°25' північної широти та між 22°08' і 40°13' східної довготи
    pub static ref UKRAINE_BBOX: Rect = Rect::new(
                Coord::from((22.08, 44.23)),
                Coord::from((40.13, 52.25)),
            );

    /// Ukraine center
    ///
    /// Центр України знаходиться в точці з географічними координатами `49°01'` північної широти і `31°02'` східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.
    pub static ref UKRAINE_CENTER: Point<f64> = Point::new(31.02, 49.01);
}
