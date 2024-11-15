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
pub struct ApiGetSwapsStreamUpdate {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<models::ApiProject>,
    #[serde(rename = "poolAddress", skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(rename = "inToken", skip_serializing_if = "Option::is_none")]
    pub in_token: Option<String>,
    #[serde(rename = "inTokenAddress", skip_serializing_if = "Option::is_none")]
    pub in_token_address: Option<String>,
    #[serde(rename = "outToken", skip_serializing_if = "Option::is_none")]
    pub out_token: Option<String>,
    #[serde(rename = "outTokenAddress", skip_serializing_if = "Option::is_none")]
    pub out_token_address: Option<String>,
    #[serde(rename = "inAmount", skip_serializing_if = "Option::is_none")]
    pub in_amount: Option<f64>,
    #[serde(rename = "outAmountMin", skip_serializing_if = "Option::is_none")]
    pub out_amount_min: Option<f64>,
    #[serde(rename = "sourceAccount", skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
    #[serde(rename = "destinationAccount", skip_serializing_if = "Option::is_none")]
    pub destination_account: Option<String>,
    #[serde(rename = "ownerAccount", skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl ApiGetSwapsStreamUpdate {
    pub fn new() -> ApiGetSwapsStreamUpdate {
        ApiGetSwapsStreamUpdate {
            success: None,
            project: None,
            pool_address: None,
            in_token: None,
            in_token_address: None,
            out_token: None,
            out_token_address: None,
            in_amount: None,
            out_amount_min: None,
            source_account: None,
            destination_account: None,
            owner_account: None,
            signature: None,
        }
    }
}

