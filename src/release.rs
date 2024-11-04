use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Release {
    #[serde(rename = "@id")]
    id: i64,
    #[serde(rename = "@status")]
    status: String,
    title: String,
    country: String,
    released: String,
    labels: Labels,
    artists: Artists,
    extraartists: Artists,
    genres: Genres,
    styles: Styles,
    master_id: MasterId,
    tracklist: TrackList,
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
pub struct Labels {
    label: Vec<Label>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Label {
    #[serde(rename = "@id")]
    id: i64,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@catno")]
    catno: String,
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

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct MasterId {
    #[serde(rename = "@is_main_release")]
    is_main_release: String,
    #[serde(rename = "$text")]
    id: i64,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TrackList {
    track: Vec<Track>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Track {
    position: String,
    title: String,
    duration: String,
}
