use super::Doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Release {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@status")]
    pub status: String,
    pub title: String,
    pub country: String,
    pub released: String,
    pub labels: Labels,
    pub artists: Artists,
    pub extraartists: Artists,
    pub genres: Genres,
    pub styles: Styles,
    pub master_id: MasterId,
    pub tracklist: TrackList,
    pub data_quality: String,
}

impl Doc for Release {
    fn index_id(&self) -> opensearch::IndexParts {
        return opensearch::IndexParts::IndexId("release", self.id.as_str());
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Artists {
    pub artist: Vec<Artist>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub anv: String,
    pub role: String,
    pub tracks: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Labels {
    pub label: Vec<Label>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Label {
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@catno")]
    pub catno: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Genres {
    pub genre: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Styles {
    pub style: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct MasterId {
    #[serde(rename = "@is_main_release")]
    pub is_main_release: bool,
    #[serde(rename = "$text")]
    pub id: i64,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TrackList {
    pub track: Vec<Track>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Track {
    pub position: String,
    pub title: String,
    pub duration: String,
}
