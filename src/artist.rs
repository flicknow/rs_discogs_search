use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Artist {
    id: i64,
    name: String,
    realname: String,
    profile: String,
    namevariations: NameVariations,
    aliases: Aliases,
    data_quality: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct NameVariations {
    name: Vec<String>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Aliases {
    #[serde(rename = "name")]
    alias: Vec<Alias>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Alias {
    #[serde(rename = "@id")]
    id: i64,
    #[serde(rename = "$text")]
    name: String,
}
