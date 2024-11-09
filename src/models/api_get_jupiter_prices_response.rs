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
pub struct ApiGetJupiterPricesResponse {
    #[serde(rename = "tokenPrices", skip_serializing_if = "Option::is_none")]
    pub token_prices: Option<Vec<models::ApiTokenPriceV2>>,
}

impl ApiGetJupiterPricesResponse {
    pub fn new() -> ApiGetJupiterPricesResponse {
        ApiGetJupiterPricesResponse {
            token_prices: None,
        }
    }
}

