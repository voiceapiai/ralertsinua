use lazy_static::lazy_static;
use ralertsinua_models::Region;

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

/// This was `SQL INSERT INTO regions VALUES` statement
/// We just use it directly in Rust
#[rustfmt::skip]
const REGIONS_DATA: [(i64, i8, &str, &str); 27] = [
    (145279, 29, "Автономна Республіка Крим", "Autonomous Republic of Crimea"),
    (142129, 4, "Волинська область", "Volyn Oblast"),
    (181453, 8, "Вінницька область", "Vinnytsia Oblast"),
    (203493, 9, "Дніпропетровська область", "Dnipropetrovsk Oblast"),
    (143947, 28, "Донецька область", "Donetsk Oblast"),
    (142491, 10, "Житомирська область", "Zhytomyr Oblast"),
    (144979, 11, "Закарпатська область", "Zakarpattia Oblast"),
    (143961, 12, "Запорізька область", "Zaporizhia Oblast"),
    (144977, 13, "Івано-Франківська область", "Ivano-Frankivsk Oblast"),
    (843733, 31, "Київ", "Kyiv"),
    (142497, 14, "Київська область", "Kyiv Oblast"),
    (203719, 15, "Кіровоградська область", "Kirovohrad Oblast"),
    (143943, 16, "Луганська область", "Luhansk Oblast"),
    (144761, 27, "Львівська область", "Lviv Oblast"),
    (145271, 17, "Миколаївська область", "Mykolaiv Oblast"),
    (145269, 18, "Одеська область", "Odesa Oblast"),
    (182589, 19, "Полтавська область", "Poltava Oblast"),
    (142473, 5, "Рівненська область", "Rivne Oblast"),
    (3148729, 30, "Севастополь", "Sevastopol"),
    (142501, 20, "Сумська область", "Sumy Oblast"),
    (145051, 21, "Тернопільська область", "Ternopil Oblast"),
    (142509, 22, "Харківська область", "Kharkiv Oblast"),
    (142045, 23, "Херсонська область", "Kherson Oblast"),
    (181485, 3, "Хмельницька область", "Khmelnytskyi Oblast"),
    (182557, 24, "Черкаська область", "Cherkasy Oblast"),
    (145053, 26, "Чернівецька область", "Chernivtsi Oblast"),
    (142499, 25, "Чернігівська область", "Chernihiv Oblast"),
];

lazy_static! {
    pub static ref UKRAINE_REGIONS: [Region; 27] = {
        let regions_arr: [Region; 27] = core::array::from_fn(|i| {
            let (osm_id, a_id, name, name_en) = REGIONS_DATA[i];
            Region {
                id: i as i8 + 1,
                osm_id,
                a_id,
                name: name.to_string(),
                name_en: name_en.to_string(),
            }
        });

        regions_arr
    };
}
