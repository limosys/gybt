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
pub struct ApiGetPriorityFeeResponse {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<models::ApiProject>,
    #[serde(rename = "percentile", skip_serializing_if = "Option::is_none")]
    pub percentile: Option<f64>,
    #[serde(rename = "feeAtPercentile", skip_serializing_if = "Option::is_none")]
    pub fee_at_percentile: Option<String>,
}

impl ApiGetPriorityFeeResponse {
    pub fn new() -> ApiGetPriorityFeeResponse {
        ApiGetPriorityFeeResponse {
            project: None,
            percentile: None,
            fee_at_percentile: None,
        }
    }
}

