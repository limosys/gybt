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
pub struct ApiStreamInfo {
    #[serde(rename = "streamName", skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "subscriptionID", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "creditUsed", skip_serializing_if = "Option::is_none")]
    pub credit_used: Option<String>,
}

impl ApiStreamInfo {
    pub fn new() -> ApiStreamInfo {
        ApiStreamInfo {
            stream_name: None,
            subscription_id: None,
            start_time: None,
            credit_used: None,
        }
    }
}
