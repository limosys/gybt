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
pub struct ApiLiquidityPoolKeys {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "baseMint", skip_serializing_if = "Option::is_none")]
    pub base_mint: Option<String>,
    #[serde(rename = "quoteMint", skip_serializing_if = "Option::is_none")]
    pub quote_mint: Option<String>,
    #[serde(rename = "lpMint", skip_serializing_if = "Option::is_none")]
    pub lp_mint: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "programID", skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(rename = "baseVault", skip_serializing_if = "Option::is_none")]
    pub base_vault: Option<String>,
    #[serde(rename = "quoteVault", skip_serializing_if = "Option::is_none")]
    pub quote_vault: Option<String>,
    #[serde(rename = "lpVault", skip_serializing_if = "Option::is_none")]
    pub lp_vault: Option<String>,
    #[serde(rename = "openOrders", skip_serializing_if = "Option::is_none")]
    pub open_orders: Option<String>,
    #[serde(rename = "targetOrders", skip_serializing_if = "Option::is_none")]
    pub target_orders: Option<String>,
    #[serde(rename = "withdrawQueue", skip_serializing_if = "Option::is_none")]
    pub withdraw_queue: Option<String>,
    #[serde(rename = "marketVersion", skip_serializing_if = "Option::is_none")]
    pub market_version: Option<i64>,
    #[serde(rename = "marketProgramID", skip_serializing_if = "Option::is_none")]
    pub market_program_id: Option<String>,
    #[serde(rename = "marketID", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "marketAuthority", skip_serializing_if = "Option::is_none")]
    pub market_authority: Option<String>,
    #[serde(rename = "marketBaseVault", skip_serializing_if = "Option::is_none")]
    pub market_base_vault: Option<String>,
    #[serde(rename = "marketQuoteVault", skip_serializing_if = "Option::is_none")]
    pub market_quote_vault: Option<String>,
    #[serde(rename = "marketBids", skip_serializing_if = "Option::is_none")]
    pub market_bids: Option<String>,
    #[serde(rename = "marketAsks", skip_serializing_if = "Option::is_none")]
    pub market_asks: Option<String>,
    #[serde(rename = "marketEventQueue", skip_serializing_if = "Option::is_none")]
    pub market_event_queue: Option<String>,
    #[serde(rename = "tradeFeeRate", skip_serializing_if = "Option::is_none")]
    pub trade_fee_rate: Option<String>,
}

impl ApiLiquidityPoolKeys {
    pub fn new() -> ApiLiquidityPoolKeys {
        ApiLiquidityPoolKeys {
            id: None,
            base_mint: None,
            quote_mint: None,
            lp_mint: None,
            version: None,
            program_id: None,
            authority: None,
            base_vault: None,
            quote_vault: None,
            lp_vault: None,
            open_orders: None,
            target_orders: None,
            withdraw_queue: None,
            market_version: None,
            market_program_id: None,
            market_id: None,
            market_authority: None,
            market_base_vault: None,
            market_quote_vault: None,
            market_bids: None,
            market_asks: None,
            market_event_queue: None,
            trade_fee_rate: None,
        }
    }
}

