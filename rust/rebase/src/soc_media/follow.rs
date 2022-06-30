use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::DID;
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

pub struct Follower {
    follows: DID,
}

impl SchemaType for Follower {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS MORE ACCURATE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "FollowCredential": "https://example.com/FollowCredential",
                "Follow": {
                    "@id": "https://example.com/Follow",
                    "@context": {
                        "follows": "https://example.com/follows",
                    }
                }
            },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "FollowCredential".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "type": ["Follow"],
            "follows": self.follows.did()?
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Ok(None)
    }
}
