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
pub struct ApiPostSettleRequestV2 {
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "baseTokenWallet")]
    pub base_token_wallet: String,
    #[serde(rename = "quoteTokenWallet")]
    pub quote_token_wallet: String,
    /// If left empty the Serum API will lookup the account which can be time consuming
    #[serde(rename = "openOrdersAddress", skip_serializing_if = "Option::is_none")]
    pub open_orders_address: Option<String>,
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

impl ApiPostSettleRequestV2 {
    pub fn new(owner_address: String, market: String, base_token_wallet: String, quote_token_wallet: String) -> ApiPostSettleRequestV2 {
        ApiPostSettleRequestV2 {
            owner_address,
            market,
            base_token_wallet,
            quote_token_wallet,
            open_orders_address: None,
            compute_limit: None,
            compute_price: None,
            tip: None,
        }
    }
}

