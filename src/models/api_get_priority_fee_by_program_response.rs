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
pub struct ApiGetPriorityFeeByProgramResponse {
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::ApiProgramPriorityFee>>,
}

impl ApiGetPriorityFeeByProgramResponse {
    pub fn new() -> ApiGetPriorityFeeByProgramResponse {
        ApiGetPriorityFeeByProgramResponse {
            data: None,
        }
    }
}
