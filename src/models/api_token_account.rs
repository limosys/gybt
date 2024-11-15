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
pub struct ApiTokenAccount {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "tokenMint", skip_serializing_if = "Option::is_none")]
    pub token_mint: Option<String>,
    #[serde(rename = "tokenAccount", skip_serializing_if = "Option::is_none")]
    pub token_account: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

impl ApiTokenAccount {
    pub fn new() -> ApiTokenAccount {
        ApiTokenAccount {
            symbol: None,
            token_mint: None,
            token_account: None,
            amount: None,
        }
    }
}

