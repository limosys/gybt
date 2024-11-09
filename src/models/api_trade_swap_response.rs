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
pub struct ApiTradeSwapResponse {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<models::ApiProject>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<models::ApiTransactionMessage>>,
    #[serde(rename = "outAmount", skip_serializing_if = "Option::is_none")]
    pub out_amount: Option<f64>,
    #[serde(rename = "outAmountMin", skip_serializing_if = "Option::is_none")]
    pub out_amount_min: Option<f64>,
    #[serde(rename = "priceImpact", skip_serializing_if = "Option::is_none")]
    pub price_impact: Option<Box<models::CommonPriceImpactPercent>>,
    #[serde(rename = "fees", skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<models::CommonFee>>,
}

impl ApiTradeSwapResponse {
    pub fn new() -> ApiTradeSwapResponse {
        ApiTradeSwapResponse {
            project: None,
            transactions: None,
            out_amount: None,
            out_amount_min: None,
            price_impact: None,
            fees: None,
        }
    }
}

