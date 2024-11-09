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
pub struct ApiMarket {
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ApiMarketStatus>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "baseMint", skip_serializing_if = "Option::is_none")]
    pub base_mint: Option<String>,
    #[serde(rename = "quotedMint", skip_serializing_if = "Option::is_none")]
    pub quoted_mint: Option<String>,
    #[serde(rename = "baseDecimals", skip_serializing_if = "Option::is_none")]
    pub base_decimals: Option<String>,
    #[serde(rename = "quoteDecimals", skip_serializing_if = "Option::is_none")]
    pub quote_decimals: Option<String>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<models::ApiProject>,
}

impl ApiMarket {
    pub fn new() -> ApiMarket {
        ApiMarket {
            market: None,
            status: None,
            address: None,
            base_mint: None,
            quoted_mint: None,
            base_decimals: None,
            quote_decimals: None,
            project: None,
        }
    }
}

