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
pub struct ApiOrderbookItemV2 {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// Serum generated OrderID
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// OpenOrders address for current market
    #[serde(rename = "ownerAddress", skip_serializing_if = "Option::is_none")]
    pub owner_address: Option<String>,
}

impl ApiOrderbookItemV2 {
    pub fn new() -> ApiOrderbookItemV2 {
        ApiOrderbookItemV2 {
            price: None,
            size: None,
            order_id: None,
            client_order_id: None,
            owner_address: None,
        }
    }
}

