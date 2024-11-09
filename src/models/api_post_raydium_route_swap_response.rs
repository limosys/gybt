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
pub struct ApiPostRaydiumRouteSwapResponse {
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<models::ApiTransactionMessage>>,
    #[serde(rename = "outAmount", skip_serializing_if = "Option::is_none")]
    pub out_amount: Option<f64>,
    #[serde(rename = "outAmountMin", skip_serializing_if = "Option::is_none")]
    pub out_amount_min: Option<f64>,
}

impl ApiPostRaydiumRouteSwapResponse {
    pub fn new() -> ApiPostRaydiumRouteSwapResponse {
        ApiPostRaydiumRouteSwapResponse {
            transactions: None,
            out_amount: None,
            out_amount_min: None,
        }
    }
}
