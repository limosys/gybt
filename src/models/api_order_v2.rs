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
pub struct ApiOrderV2 {
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    ///  Supported values : Bid, Ask
    #[serde(rename = "side")]
    pub side: String,
    /// Supported values : Limit, IoC, PostOnly
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "remainingSize", skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<f64>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "openOrderAccount", skip_serializing_if = "Option::is_none")]
    pub open_order_account: Option<String>,
}

impl ApiOrderV2 {
    pub fn new(side: String, r#type: String) -> ApiOrderV2 {
        ApiOrderV2 {
            order_id: None,
            market: None,
            side,
            r#type,
            price: None,
            remaining_size: None,
            created_at: None,
            client_order_id: None,
            open_order_account: None,
        }
    }
}
