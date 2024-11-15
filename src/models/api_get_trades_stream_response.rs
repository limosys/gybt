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
pub struct ApiGetTradesStreamResponse {
    #[serde(rename = "slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Box<models::ApiGetTradesResponse>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl ApiGetTradesStreamResponse {
    pub fn new() -> ApiGetTradesStreamResponse {
        ApiGetTradesStreamResponse {
            slot: None,
            trades: None,
            timestamp: None,
        }
    }
}

