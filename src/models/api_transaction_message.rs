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
pub struct ApiTransactionMessage {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "isCleanup", skip_serializing_if = "Option::is_none")]
    pub is_cleanup: Option<bool>,
}

impl ApiTransactionMessage {
    pub fn new() -> ApiTransactionMessage {
        ApiTransactionMessage {
            content: None,
            is_cleanup: None,
        }
    }
}
