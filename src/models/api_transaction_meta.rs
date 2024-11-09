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
pub struct ApiTransactionMeta {
    #[serde(rename = "err", skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "errored", skip_serializing_if = "Option::is_none")]
    pub errored: Option<bool>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename = "preBalances", skip_serializing_if = "Option::is_none")]
    pub pre_balances: Option<Vec<String>>,
    #[serde(rename = "postBalances", skip_serializing_if = "Option::is_none")]
    pub post_balances: Option<Vec<String>>,
    #[serde(rename = "innerInstructions", skip_serializing_if = "Option::is_none")]
    pub inner_instructions: Option<Vec<models::ApiTransactionMetaInnerInstruction>>,
    #[serde(rename = "logMessages", skip_serializing_if = "Option::is_none")]
    pub log_messages: Option<Vec<String>>,
    #[serde(rename = "preTokenBalances", skip_serializing_if = "Option::is_none")]
    pub pre_token_balances: Option<Vec<models::ApiTransactionMetaTokenBalance>>,
    #[serde(rename = "postTokenBalances", skip_serializing_if = "Option::is_none")]
    pub post_token_balances: Option<Vec<models::ApiTransactionMetaTokenBalance>>,
}

impl ApiTransactionMeta {
    pub fn new() -> ApiTransactionMeta {
        ApiTransactionMeta {
            err: None,
            errored: None,
            fee: None,
            pre_balances: None,
            post_balances: None,
            inner_instructions: None,
            log_messages: None,
            pre_token_balances: None,
            post_token_balances: None,
        }
    }
}

