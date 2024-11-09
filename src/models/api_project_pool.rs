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
pub struct ApiProjectPool {
    #[serde(rename = "pool", skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(rename = "poolAddress", skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(rename = "token1Reserves", skip_serializing_if = "Option::is_none")]
    pub token1_reserves: Option<String>,
    #[serde(rename = "token1MintAddress", skip_serializing_if = "Option::is_none")]
    pub token1_mint_address: Option<String>,
    #[serde(rename = "token1MintSymbol", skip_serializing_if = "Option::is_none")]
    pub token1_mint_symbol: Option<String>,
    #[serde(rename = "token2Reserves", skip_serializing_if = "Option::is_none")]
    pub token2_reserves: Option<String>,
    #[serde(rename = "token2MintAddress", skip_serializing_if = "Option::is_none")]
    pub token2_mint_address: Option<String>,
    #[serde(rename = "token2MintSymbol", skip_serializing_if = "Option::is_none")]
    pub token2_mint_symbol: Option<String>,
    #[serde(rename = "openTime", skip_serializing_if = "Option::is_none")]
    pub open_time: Option<String>,
    #[serde(rename = "poolType", skip_serializing_if = "Option::is_none")]
    pub pool_type: Option<String>,
    #[serde(rename = "liquidityPoolKeys", skip_serializing_if = "Option::is_none")]
    pub liquidity_pool_keys: Option<Box<models::ApiLiquidityPoolKeys>>,
}

impl ApiProjectPool {
    pub fn new() -> ApiProjectPool {
        ApiProjectPool {
            pool: None,
            pool_address: None,
            token1_reserves: None,
            token1_mint_address: None,
            token1_mint_symbol: None,
            token2_reserves: None,
            token2_mint_address: None,
            token2_mint_symbol: None,
            open_time: None,
            pool_type: None,
            liquidity_pool_keys: None,
        }
    }
}

