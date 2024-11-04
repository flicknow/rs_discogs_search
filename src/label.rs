use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub profile: String,
    pub sublabels: SubLabels,
    pub data_quality: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SubLabels {
    #[serde(rename = "label")]
    pub sublabel: Vec<SubLabel>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SubLabel {
    #[serde(rename = "@id")]
    pub id: i64,
    #[serde(rename = "$text")]
    pub label: String,
}
