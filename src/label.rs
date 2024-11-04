use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Label {
    id: i64,
    name: String,
    profile: String,
    sublabels: SubLabels,
    data_quality: String,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SubLabels {
    #[serde(rename = "label")]
    sublabel: Vec<SubLabel>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SubLabel {
    #[serde(rename = "@id")]
    id: i64,
    #[serde(rename = "$text")]
    label: String,
}
