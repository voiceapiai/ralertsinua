use serde::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Region {
    pub id: i8,
    pub a_id: i8,
    pub osm_id: i64,
    // #[sqlx(rename = "geo", default)]
    // pub geo: String,
    pub name: String,
    pub name_en: String,
}

#[derive(Debug, Default, Deserialize, sqlx::FromRow)]
pub struct RegionGeo {
    pub osm_id: i64,
    pub a_id: i8,
    pub geo: String,
}