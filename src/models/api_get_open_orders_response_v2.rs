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
pub struct ApiGetOpenOrdersResponseV2 {
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<models::ApiOrderV2>>,
}

impl ApiGetOpenOrdersResponseV2 {
    pub fn new() -> ApiGetOpenOrdersResponseV2 {
        ApiGetOpenOrdersResponseV2 {
            orders: None,
        }
    }
}
