use lazy_static::lazy_static;
use ralertsinua_models::region::*;

/// Ukraine borders represented as Polygon in WKT file
pub const UKRAINE_BORDERS_POYGON_WKT: &str = include_str!("../../assets/ukraine.wkt");
/// Ukraine bounding box coords tuple - (min_x, min_y), (max_x, max_y)
///
/// <em>Територія України розташована між 44°23' і 52°25' північної широти та між 22°08' і 40°13' східної довготи</em>
pub const UKRAINE_BBOX: [(f64, f64); 2] = [(22.08, 44.23), (40.13, 52.25)];
/// Ukraine center
///
/// <em>Центр України знаходиться в точці з географічними координатами `49°01'` північної широти і `31°02'` східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.</em>
#[allow(unused)]
pub const UKRAINE_CENTER: (f64, f64) = (49.01, 31.02);

lazy_static! {
    pub static ref UKRAINE_REGIONS: [Region; 27] = {
        let regions_arr: [Region; 27] = core::array::from_fn(|i| {
            let (osm_id, a_id, name, name_en) = REGIONS_DATA[i];
            Region {
                id: i as i32 + 1,
                osm_id,
                a_id,
                name: name.to_string(),
                name_en: name_en.to_string(),
                geom: "".to_string(),
            }
        });

        regions_arr
    };
}
