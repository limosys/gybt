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
pub struct ApiPostRaydiumCpmmSwapResponse {
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Box<models::ApiTransactionMessage>>,
    #[serde(rename = "outAmount", skip_serializing_if = "Option::is_none")]
    pub out_amount: Option<f64>,
    #[serde(rename = "outAmountMin", skip_serializing_if = "Option::is_none")]
    pub out_amount_min: Option<f64>,
}

impl ApiPostRaydiumCpmmSwapResponse {
    pub fn new() -> ApiPostRaydiumCpmmSwapResponse {
        ApiPostRaydiumCpmmSwapResponse {
            transaction: None,
            out_amount: None,
            out_amount_min: None,
        }
    }
}

