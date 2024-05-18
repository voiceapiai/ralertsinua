[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=voiceapiai_alertsinua-cli&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=voiceapiai_alertsinua-cli) [![Coverage Status](https://coveralls.io/repos/github/voiceapiai/ralertsinua/badge.svg)](https://coveralls.io/github/voiceapiai/ralertsinua) ![Crates.io Version](https://img.shields.io/crates/v/ralertsinua-http) ![Crates.io License](https://img.shields.io/crates/l/ralertsinua-http) ![docs.rs](https://img.shields.io/docsrs/ralertsinua-http)


# ralertsinua-geo

<p>Rust async API wrapper (<em>reqwest</em>) & <abbr title="Terminal User Interface">TUI</abbr> (<em>ratatui</em>) for <u>alerts.in.ua</u>

![screencast](https://raw.githubusercontent.com/voiceapiai/ralertsinua/main/docs/assets/screencast.gif)

#

## Introduction
The Alerts.in.ua API Client is a Rust library that simplifies access to the alerts.in.ua API service. It provides real-time information about air raid alerts and other potential threats.



## Installation
To install the Alerts.in.ua API Client, run the following command in your terminal:

```bash
cargo add ralertsinua-geo
```

## Usage

⚠️ Before you can use this library, you need to obtain an API token by submitting an [API request form](https://alerts.in.ua/api-request).

Here's an basic example of how to use the library to get a list of active alerts:

Async:
```rs
use ralertsinua_geo::AlertsInUaGeo;
// Initialize the client
geo_client = AlertsInUaGeo();

/// The API for the AlertsInUaClient
pub trait AlertsInUaGeo: WithBoundingRect + Sync + Send + core::fmt::Debug {
    fn boundary(&self) -> CountryBoundary;
    fn locations(&self) -> [Location; 27];
    fn get_location_by_uid(&self, uid: i32) -> Option<Location>;
    fn get_location_by_name(&self, name: &str) -> Option<Location>;
}
```

## Location

The `Location` struct represents a Ukraine's administrative unit lv4

```rs
/// Ukraine's administrative unit lv4  - *oblast*
#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Location {
    /// OSM Relation Id
    pub relation_id: String,
    /// Alerts.in.ua "uid"
    pub location_uid: i32,
    /// "state" or "city" or "special"
    pub location_type: String,
    /// Geometry for boundary (Polygon or MultiPolygon)
    pub geometry: Geometry,
    /// Name in uk
    pub name: String,
    /// Name in en
    pub name_en: String,

    /// And some functions from traits
    fn geometry(&self) -> &Geometry;
    fn boundary(&self) -> &Polygon;
    fn center(&self) -> (f64, f64);
    /// To be used in TUI when , implements `Shape`
    fn draw(&self, painter: &mut Painter);
```

## License
MIT 2024

*[TUI]: Terminal User Interface
