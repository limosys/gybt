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
pub struct ApiTokenPriceV2 {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "tokenAddress", skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: Option<f64>,
    #[serde(rename = "buySize", skip_serializing_if = "Option::is_none")]
    pub buy_size: Option<f64>,
    #[serde(rename = "sell", skip_serializing_if = "Option::is_none")]
    pub sell: Option<f64>,
    #[serde(rename = "sellSize", skip_serializing_if = "Option::is_none")]
    pub sell_size: Option<f64>,
}

impl ApiTokenPriceV2 {
    pub fn new() -> ApiTokenPriceV2 {
        ApiTokenPriceV2 {
            token: None,
            token_address: None,
            buy: None,
            buy_size: None,
            sell: None,
            sell_size: None,
        }
    }
}

