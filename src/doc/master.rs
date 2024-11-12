use super::Doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Master {
    #[serde(rename = "@id")]
    pub id: String,
    pub artists: Artists,
    pub genres: Genres,
    pub styles: Styles,
    pub year: i64,
    pub title: String,
    pub data_quality: String,
}

impl Doc for Master {
    fn index_id(&self) -> opensearch::IndexParts {
        return opensearch::IndexParts::IndexId("master", self.id.as_str());
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
pub struct Genres {
    pub genre: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Styles {
    pub style: Vec<String>,
}
