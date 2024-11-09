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
pub struct ApiGetQuotesResponse {
    #[serde(rename = "inToken", skip_serializing_if = "Option::is_none")]
    pub in_token: Option<String>,
    #[serde(rename = "inTokenAddress", skip_serializing_if = "Option::is_none")]
    pub in_token_address: Option<String>,
    #[serde(rename = "outToken", skip_serializing_if = "Option::is_none")]
    pub out_token: Option<String>,
    #[serde(rename = "outTokenAddress", skip_serializing_if = "Option::is_none")]
    pub out_token_address: Option<String>,
    #[serde(rename = "inAmount", skip_serializing_if = "Option::is_none")]
    pub in_amount: Option<f64>,
    #[serde(rename = "quotes", skip_serializing_if = "Option::is_none")]
    pub quotes: Option<Vec<models::ApiProjectQuote>>,
}

impl ApiGetQuotesResponse {
    pub fn new() -> ApiGetQuotesResponse {
        ApiGetQuotesResponse {
            in_token: None,
            in_token_address: None,
            out_token: None,
            out_token_address: None,
            in_amount: None,
            quotes: None,
        }
    }
}

