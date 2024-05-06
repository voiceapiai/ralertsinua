use serde::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub id: i32,
    pub a_id: i32,
    pub osm_id: i32,
    pub name: String,
    pub name_en: String,
    #[serde(default)]
    pub geom: String,
}

/// This was `SQL INSERT INTO regions VALUES` statement
/// We just use it directly in Rust
#[rustfmt::skip]
pub const REGIONS_DATA: [(i32, i32, &str, &str); 27] = [
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