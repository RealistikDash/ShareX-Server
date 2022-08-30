use serde::{Deserialize as Deserialise, Serialize as Serialise};

#[derive(Deserialise, Serialise)]
pub struct SharexHeaderConfig {
    pub Authorisation: String,
}

#[derive(Deserialise, Serialise)]
pub struct ShareXConfig {
    pub Version: String,
    pub DestinationType: String,
    pub RequestMethod: String,
    pub RequestURL: String,
    pub Headers: SharexHeaderConfig,
    pub Body: String,
    pub FileFormName: String,
    pub URL: String,
    pub ThumbnailURL: String,
    pub DeletionURL: String,
    pub ErrorMessage: String,
}
