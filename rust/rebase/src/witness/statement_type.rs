use crate::witness::{
    dns::Claim as DnsStatement, github::Opts as GitHubStatement,
    self_signed::KeyToKeyLink as SelfSignedStatement,
    soc_media::follow::generate_statement as generate_follow, twitter::Opts as TwitterStatement,
    witness::Statement,
};

use crate::soc_media::key_to_key_link::KeyToKeyLink;

use crate::signer::signer::{SignerError, SignerType};
use crate::witness::signer_type::SignerTypes;
use crate::witness::witness::WitnessError;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum StatementTypes {
    #[serde(rename = "dns")]
    Dns(DnsStatement),
    #[serde(rename = "follow")]
    Follow(KeyToKeyLink),
    #[serde(rename = "github")]
    GitHub(GitHubStatement),
    #[serde(rename = "self_signed")]
    SelfSigned(SelfSignedStatement),
    #[serde(rename = "twitter")]
    Twitter(TwitterStatement),
}

impl Statement for StatementTypes {
    fn generate_statement(&self) -> Result<String, WitnessError> {
        match &self {
            StatementTypes::Dns(x) => x.generate_statement(),
            StatementTypes::Follow(x) => generate_follow(x),
            StatementTypes::GitHub(x) => x.generate_statement(),
            StatementTypes::SelfSigned(x) => x.generate_self_signed(),
            StatementTypes::Twitter(x) => x.generate_statement(),
        }
    }

    fn delimitor(&self) -> String {
        match &self {
            StatementTypes::Dns(x) => x.delimitor(),
            StatementTypes::GitHub(x) => x.delimitor(),
            StatementTypes::Twitter(x) => x.delimitor(),

            // TODO / NOTE: Should these be an err? Permitted? A value?
            StatementTypes::Follow(_) => String::new(),
            StatementTypes::SelfSigned(_) => String::new(),
        }
    }

    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        match &self {
            StatementTypes::Dns(x) => x.signer_type(),
            StatementTypes::GitHub(x) => x.signer_type(),
            StatementTypes::Follow(x) => SignerTypes::new(&x.key_1),
            StatementTypes::Twitter(x) => x.signer_type(),
            // TODO: Should this be seperated into a different trait?
            StatementTypes::SelfSigned(_) => Err(SignerError::InvalidId {
                signer_type: "2 key".to_owned(),
                reason: "cannot call signer_type on 2 key statement opts".to_owned(),
            }),
        }
    }
}
