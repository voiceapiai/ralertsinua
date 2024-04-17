#![allow(unused)]
use ratatui::style::{palette::tailwind, Color};

// Map
/// Ukraine bounding box coords tuple - (min_x, min_y), (max_x, max_y)
///
/// <em>Територія України розташована між 44°23' і 52°25' північної широти та між 22°08' і 40°13' східної довготи</em>
pub const BOUNDINGBOX: [(f64, f64); 2] = [(22.08, 44.23), (40.13, 52.25)];
/// Ukraine center
///
/// <em>Центр України знаходиться в точці з географічними координатами `49°01'` північної широти і `31°02'` східної довготи. Ця точка розміщена за 2 км на захід від м. Ватутіного у Черкаській області – с. Мар'янівка. За іншою версією – с. Добровеличківка Кіровоградської області.</em>
pub const CENTER: (f64, f64) = (49.01, 31.02);

pub const PADDING: f64 = 0.5;
// Colors
pub const MARKER_COLOR: Color = tailwind::YELLOW.c950;
pub const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
pub const ALERT_ROW_COLOR: Color = tailwind::RED.c900;
pub const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
pub const TEXT_COLOR: Color = tailwind::SLATE.c200;
pub const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;
// Alerts common
// const A_REGIONS_STATUS_RESPONSE_EXAMPLE: ArrayString<27> = ArrayString::<27>::from("ANNNNNNNNNNNANNNNNNNNNNNNNN");
pub const A_REGIONS_IDS: [i8; 27] = [
    29, 4, 8, 9, 28, 10, 11, 12, 13, 31, 14, 15, 16, 27, 17, 18, 19, 5, 30, 20, 21, 22, 23, 3, 24,
    26, 25,
];
