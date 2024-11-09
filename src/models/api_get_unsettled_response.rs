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
pub struct ApiGetUnsettledResponse {
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "unsettled", skip_serializing_if = "Option::is_none")]
    pub unsettled: Option<Vec<models::ApiUnsettledAccount>>,
}

impl ApiGetUnsettledResponse {
    pub fn new() -> ApiGetUnsettledResponse {
        ApiGetUnsettledResponse {
            market: None,
            unsettled: None,
        }
    }
}

