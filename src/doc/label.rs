use super::Doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub profile: String,
    pub sublabels: SubLabels,
    pub data_quality: String,
}

impl Doc for Label {
    fn index_id(&self) -> opensearch::IndexParts {
        return opensearch::IndexParts::IndexId("label", self.id.as_str());
    }
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
