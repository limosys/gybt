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
pub struct ApiGetOrderbooksStreamResponse {
    #[serde(rename = "slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(rename = "orderbook", skip_serializing_if = "Option::is_none")]
    pub orderbook: Option<Box<models::ApiGetOrderbookResponse>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl ApiGetOrderbooksStreamResponse {
    pub fn new() -> ApiGetOrderbooksStreamResponse {
        ApiGetOrderbooksStreamResponse {
            slot: None,
            orderbook: None,
            timestamp: None,
        }
    }
}

