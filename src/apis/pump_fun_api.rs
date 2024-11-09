/*
 * Trader API
 *
 * Easy-to-use API for interacting with trader services on the Solana blockchain, powered by bloXroute Labs.
 *
 * The version of the OpenAPI document: 1.6
 * Contact: support@bloxroute.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`api_get_pump_fun_quotes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiGetPumpFunQuotesError {
    DefaultResponse(models::RpcStatus),
    UnknownValue(serde_json::Value),
}


/// Returns quotes from PumpFun
pub async fn api_get_pump_fun_quotes(configuration: &configuration::Configuration, quote_type: Option<&str>, mint_address: Option<&str>, bonding_curve_address: Option<&str>, amount: Option<f64>, slippage: Option<f64>) -> Result<models::ApiGetPumpFunQuotesResponse, Error<ApiGetPumpFunQuotesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v2/pumpfun/quotes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = quote_type {
        local_var_req_builder = local_var_req_builder.query(&[("quoteType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = mint_address {
        local_var_req_builder = local_var_req_builder.query(&[("mintAddress", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = bonding_curve_address {
        local_var_req_builder = local_var_req_builder.query(&[("bondingCurveAddress", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = amount {
        local_var_req_builder = local_var_req_builder.query(&[("amount", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = slippage {
        local_var_req_builder = local_var_req_builder.query(&[("slippage", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ApiGetPumpFunQuotesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
