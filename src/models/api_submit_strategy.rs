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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiSubmitStrategy {
    #[serde(rename = "P_UKNOWN")]
    Uknown,
    #[serde(rename = "P_SUBMIT_ALL")]
    SubmitAll,
    #[serde(rename = "P_ABORT_ON_FIRST_ERROR")]
    AbortOnFirstError,
    #[serde(rename = "P_WAIT_FOR_CONFIRMATION")]
    WaitForConfirmation,

}

impl std::fmt::Display for ApiSubmitStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Uknown => write!(f, "P_UKNOWN"),
            Self::SubmitAll => write!(f, "P_SUBMIT_ALL"),
            Self::AbortOnFirstError => write!(f, "P_ABORT_ON_FIRST_ERROR"),
            Self::WaitForConfirmation => write!(f, "P_WAIT_FOR_CONFIRMATION"),
        }
    }
}

impl Default for ApiSubmitStrategy {
    fn default() -> ApiSubmitStrategy {
        Self::Uknown
    }
}

