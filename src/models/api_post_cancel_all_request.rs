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
pub struct ApiPostCancelAllRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "openOrdersAddresses", skip_serializing_if = "Option::is_none")]
    pub open_orders_addresses: Option<Vec<String>>,
    /// Optional: specifies total compute limit to be allocated for all instructions within the created tx
    #[serde(rename = "computeLimit", skip_serializing_if = "Option::is_none")]
    pub compute_limit: Option<i64>,
    /// Optional: specifies compute price in microlamports to be included as a part of instruction, known as priority fee
    #[serde(rename = "computePrice", skip_serializing_if = "Option::is_none")]
    pub compute_price: Option<String>,
    /// Optional: Specifies a tip amount that will be used to pay for front-running protection and/or bundle submission services. Minimum value is 1025
    #[serde(rename = "tip", skip_serializing_if = "Option::is_none")]
    pub tip: Option<String>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<models::ApiProject>,
}

impl ApiPostCancelAllRequest {
    pub fn new(market: String, owner_address: String) -> ApiPostCancelAllRequest {
        ApiPostCancelAllRequest {
            market,
            owner_address,
            open_orders_addresses: None,
            compute_limit: None,
            compute_price: None,
            tip: None,
            project: None,
        }
    }
}

