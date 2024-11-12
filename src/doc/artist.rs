use super::Doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub realname: String,
    pub profile: String,
    pub namevariations: NameVariations,
    pub aliases: Aliases,
    pub data_quality: String,
}

impl Doc for Artist {
    fn index_id(&self) -> opensearch::IndexParts {
        return opensearch::IndexParts::IndexId("artist", self.id.as_str());
    }
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct NameVariations {
    pub name: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Aliases {
    #[serde(rename = "name")]
    pub alias: Vec<Alias>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Alias {
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "$text")]
    pub name: String,
}
