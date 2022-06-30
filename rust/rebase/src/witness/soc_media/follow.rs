use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::SignerType;
use crate::soc_media::key_to_key_link::KeyToKeyLink;
use crate::witness::signer_type::SignerTypes;
use crate::witness::witness::WitnessError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, Serialize)]
pub struct Claim {
    pub statement_opts: KeyToKeyLink,
    pub signature: String,
}

pub fn generate_statement(statement_opts: &KeyToKeyLink) -> Result<String, WitnessError> {
    Ok(format!(
        "{} follows {}",
        statement_opts.key_1.did()?,
        statement_opts.key_2.did()?
    ))
}

impl Claim {
    pub async fn new(
        statement_opts: KeyToKeyLink,
        signature: String,
    ) -> Result<Self, WitnessError> {
        let st = SignerTypes::new(&statement_opts.key_1)?;
        st.valid_signature(&generate_statement(&statement_opts)?, &signature)
            .await?;
        Ok(Claim {
            statement_opts,
            signature,
        })
    }
}

impl SchemaType for Claim {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS MORE ACCURATE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "WitnessFollowCredential": "https://example.com/FollowCredential",
                "WitnessFollowSubject": {
                    "@id": "https://example.com/WitnessFollowSubject",
                    "@context": {
                        "follower": "https://example.com/follower",
                        "follows": "https://example.com/follows",
                    }
                },
                "WitnessFollowEvidence": {
                    "@id": "https://example.com/WitnessFollowEvidence",
                    "@context": {
                        "follower_signature": "https://example.com/follower_signature",
                        "statement": "https://example.com/statement"
                    }
                }
            },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "WitnessFollowCredential".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "type": ["WitnessFollowSubject"],
            "follower": self.statement_opts.key_1.did()?,
            "following": self.statement_opts.key_2.did()?,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "statement".to_string(),
            serde_json::Value::String(generate_statement(&self.statement_opts).map_err(|e| {
                SchemaError::BadSubject(format!("failed in statement generation: {}", e))
            })?),
        );

        evidence_map.insert(
            "follower_signature".to_string(),
            serde_json::Value::String(self.signature.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["WitnessFollowEvidence".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }
}
