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
pub struct ApiOrder {
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<models::ApiSide>,
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<models::CommonOrderType>>,
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

impl ApiOrder {
    pub fn new() -> ApiOrder {
        ApiOrder {
            order_id: None,
            market: None,
            side: None,
            types: None,
            price: None,
            remaining_size: None,
            created_at: None,
            client_order_id: None,
            open_order_account: None,
        }
    }
}
