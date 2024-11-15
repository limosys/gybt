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
pub struct ApiGetOrderbookResponse {
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "marketAddress", skip_serializing_if = "Option::is_none")]
    pub market_address: Option<String>,
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<models::ApiOrderbookItem>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<models::ApiOrderbookItem>>,
}

impl ApiGetOrderbookResponse {
    pub fn new() -> ApiGetOrderbookResponse {
        ApiGetOrderbookResponse {
            market: None,
            market_address: None,
            bids: None,
            asks: None,
        }
    }
}

