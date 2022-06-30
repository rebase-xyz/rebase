use crate::signer::signer::DID;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct KeyToKeyLink {
    pub key_1: DID,
    pub key_2: DID,
}