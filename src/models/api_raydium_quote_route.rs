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
pub struct ApiRaydiumQuoteRoute {
    #[serde(rename = "inAmount", skip_serializing_if = "Option::is_none")]
    pub in_amount: Option<f64>,
    #[serde(rename = "outAmount", skip_serializing_if = "Option::is_none")]
    pub out_amount: Option<f64>,
    #[serde(rename = "outAmountMin", skip_serializing_if = "Option::is_none")]
    pub out_amount_min: Option<f64>,
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<models::ApiRaydiumQuoteStep>>,
}

impl ApiRaydiumQuoteRoute {
    pub fn new() -> ApiRaydiumQuoteRoute {
        ApiRaydiumQuoteRoute {
            in_amount: None,
            out_amount: None,
            out_amount_min: None,
            steps: None,
        }
    }
}
