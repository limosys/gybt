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
pub struct ApiGetPricesStreamResponse {
    #[serde(rename = "slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<models::ApiTokenPrice>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl ApiGetPricesStreamResponse {
    pub fn new() -> ApiGetPricesStreamResponse {
        ApiGetPricesStreamResponse {
            slot: None,
            price: None,
            timestamp: None,
        }
    }
}
