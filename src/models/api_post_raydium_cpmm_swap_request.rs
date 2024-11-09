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
pub struct ApiPostRaydiumCpmmSwapRequest {
    #[serde(rename = "ownerAddress", skip_serializing_if = "Option::is_none")]
    pub owner_address: Option<String>,
    #[serde(rename = "inToken", skip_serializing_if = "Option::is_none")]
    pub in_token: Option<String>,
    #[serde(rename = "outToken", skip_serializing_if = "Option::is_none")]
    pub out_token: Option<String>,
    #[serde(rename = "inAmount", skip_serializing_if = "Option::is_none")]
    pub in_amount: Option<f64>,
    #[serde(rename = "slippage", skip_serializing_if = "Option::is_none")]
    pub slippage: Option<f64>,
    #[serde(rename = "poolAddress", skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    /// Optional: specifies total compute limit to be allocated for all instructions within the created tx
    #[serde(rename = "computeLimit", skip_serializing_if = "Option::is_none")]
    pub compute_limit: Option<i64>,
    /// Optional: specifies compute price in microlamports to be included as a part of instruction, known as priority fee
    #[serde(rename = "computePrice", skip_serializing_if = "Option::is_none")]
    pub compute_price: Option<String>,
    /// Optional: Specifies a tip amount that will be used to pay for front-running protection and/or bundle submission services. Minimum value is 1025
    #[serde(rename = "tip", skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,
}

impl ApiPostRaydiumCpmmSwapRequest {
    pub fn new() -> ApiPostRaydiumCpmmSwapRequest {
        ApiPostRaydiumCpmmSwapRequest {
            owner_address: None,
            in_token: None,
            out_token: None,
            in_amount: None,
            slippage: None,
            pool_address: None,
            compute_limit: None,
            compute_price: None,
            tip: None,
        }
    }
}

