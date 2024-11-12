use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub mod artist;
pub mod label;
pub mod master;
pub mod release;

pub trait Doc: DeserializeOwned + Serialize + Debug + Send + Sync {
    fn index_id(&self) -> opensearch::IndexParts;
}
