use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Master {
    #[serde(rename = "@id")]
    id: i64,
    artists: Artists,
    genres: Genres,
    styles: Styles,
    year: i64,
    title: String,
    data_quality: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Artists {
    artist: Vec<Artist>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Artist {
    id: i64,
    name: String,
    anv: String,
    role: String,
    tracks: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Genres {
    genre: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Styles {
    style: Vec<String>,
}
