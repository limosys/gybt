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
pub struct ApiJupiterQuoteStep {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::ApiStepProject>>,
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
    #[serde(rename = "outAmount", skip_serializing_if = "Option::is_none")]
    pub out_amount: Option<f64>,
    #[serde(rename = "slippage", skip_serializing_if = "Option::is_none")]
    pub slippage: Option<f64>,
    #[serde(rename = "priceImpactPercent", skip_serializing_if = "Option::is_none")]
    pub price_impact_percent: Option<Box<models::CommonPriceImpactPercentV2>>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<Box<models::CommonFee>>,
    #[serde(rename = "outAmountMin", skip_serializing_if = "Option::is_none")]
    pub out_amount_min: Option<f64>,
}

impl ApiJupiterQuoteStep {
    pub fn new() -> ApiJupiterQuoteStep {
        ApiJupiterQuoteStep {
            project: None,
            in_token: None,
            in_token_address: None,
            out_token: None,
            out_token_address: None,
            in_amount: None,
            out_amount: None,
            slippage: None,
            price_impact_percent: None,
            fee: None,
            out_amount_min: None,
        }
    }
}

