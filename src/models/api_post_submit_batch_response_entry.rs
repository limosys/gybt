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
pub struct ApiPostSubmitBatchResponseEntry {
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "submitted", skip_serializing_if = "Option::is_none")]
    pub submitted: Option<bool>,
}

impl ApiPostSubmitBatchResponseEntry {
    pub fn new() -> ApiPostSubmitBatchResponseEntry {
        ApiPostSubmitBatchResponseEntry {
            signature: None,
            error: None,
            submitted: None,
        }
    }
}
