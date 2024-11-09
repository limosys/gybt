/*
 * Trader API
 *
 * Easy-to-use API for interacting with trader services on the Solana blockchain, powered by bloXroute Labs.
 *
 * The version of the OpenAPI document: 1.6
 * Contact: support@bloxroute.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPublicKeys {
    #[serde(rename = "pks", skip_serializing_if = "Option::is_none")]
    pub pks: Option<Vec<String>>,
}

impl ApiPublicKeys {
    pub fn new() -> ApiPublicKeys {
        ApiPublicKeys {
            pks: None,
        }
    }
}

