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
pub struct ApiProgramPriorityFee {
    #[serde(rename = "ten", skip_serializing_if = "Option::is_none")]
    pub ten: Option<String>,
    #[serde(rename = "hundred", skip_serializing_if = "Option::is_none")]
    pub hundred: Option<String>,
    #[serde(rename = "fifteen", skip_serializing_if = "Option::is_none")]
    pub fifteen: Option<String>,
    #[serde(rename = "twenty", skip_serializing_if = "Option::is_none")]
    pub twenty: Option<String>,
    #[serde(rename = "twentyFive", skip_serializing_if = "Option::is_none")]
    pub twenty_five: Option<String>,
    #[serde(rename = "thirty", skip_serializing_if = "Option::is_none")]
    pub thirty: Option<String>,
    #[serde(rename = "thirtyFive", skip_serializing_if = "Option::is_none")]
    pub thirty_five: Option<String>,
    #[serde(rename = "forty", skip_serializing_if = "Option::is_none")]
    pub forty: Option<String>,
    #[serde(rename = "fortyFive", skip_serializing_if = "Option::is_none")]
    pub forty_five: Option<String>,
    #[serde(rename = "five", skip_serializing_if = "Option::is_none")]
    pub five: Option<String>,
    #[serde(rename = "fifty", skip_serializing_if = "Option::is_none")]
    pub fifty: Option<String>,
    #[serde(rename = "fiftyFive", skip_serializing_if = "Option::is_none")]
    pub fifty_five: Option<String>,
    #[serde(rename = "sixty", skip_serializing_if = "Option::is_none")]
    pub sixty: Option<String>,
    #[serde(rename = "sixtyFive", skip_serializing_if = "Option::is_none")]
    pub sixty_five: Option<String>,
    #[serde(rename = "seventy", skip_serializing_if = "Option::is_none")]
    pub seventy: Option<String>,
    #[serde(rename = "seventyFive", skip_serializing_if = "Option::is_none")]
    pub seventy_five: Option<String>,
    #[serde(rename = "eighty", skip_serializing_if = "Option::is_none")]
    pub eighty: Option<String>,
    #[serde(rename = "eightyFive", skip_serializing_if = "Option::is_none")]
    pub eighty_five: Option<String>,
    #[serde(rename = "ninety", skip_serializing_if = "Option::is_none")]
    pub ninety: Option<String>,
    #[serde(rename = "ninetyFive", skip_serializing_if = "Option::is_none")]
    pub ninety_five: Option<String>,
    #[serde(rename = "program", skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
}

impl ApiProgramPriorityFee {
    pub fn new() -> ApiProgramPriorityFee {
        ApiProgramPriorityFee {
            ten: None,
            hundred: None,
            fifteen: None,
            twenty: None,
            twenty_five: None,
            thirty: None,
            thirty_five: None,
            forty: None,
            forty_five: None,
            five: None,
            fifty: None,
            fifty_five: None,
            sixty: None,
            sixty_five: None,
            seventy: None,
            seventy_five: None,
            eighty: None,
            eighty_five: None,
            ninety: None,
            ninety_five: None,
            program: None,
        }
    }
}
