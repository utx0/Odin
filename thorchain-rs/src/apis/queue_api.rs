/*
 * Thornode API
 *
 * Thornode REST API.
 *
 * The version of the OpenAPI document: 1.119.0
 * Contact: devs@thorchain.org
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`queue`]
#[derive(Clone, Debug, Default)]
pub struct QueueParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`queue_outbound`]
#[derive(Clone, Debug, Default)]
pub struct QueueOutboundParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`queue_scheduled`]
#[derive(Clone, Debug, Default)]
pub struct QueueScheduledParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`queue_swap`]
#[derive(Clone, Debug, Default)]
pub struct QueueSwapParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}


/// struct for typed errors of method [`queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueueError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`queue_outbound`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueueOutboundError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`queue_scheduled`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueueScheduledError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`queue_swap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueueSwapError {
    UnknownValue(serde_json::Value),
}


/// Returns queue statistics.
pub async fn queue(configuration: &configuration::Configuration, params: QueueParams) -> Result<crate::models::QueueResponse, Error<QueueError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/queue", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = height {
        local_var_req_builder = local_var_req_builder.query(&[("height", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QueueError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the outbound queue including estimated RUNE values.
pub async fn queue_outbound(configuration: &configuration::Configuration, params: QueueOutboundParams) -> Result<Vec<crate::models::TxOutItem>, Error<QueueOutboundError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/queue/outbound", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = height {
        local_var_req_builder = local_var_req_builder.query(&[("height", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QueueOutboundError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the scheduled queue.
pub async fn queue_scheduled(configuration: &configuration::Configuration, params: QueueScheduledParams) -> Result<Vec<crate::models::TxOutItem>, Error<QueueScheduledError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/queue/scheduled", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = height {
        local_var_req_builder = local_var_req_builder.query(&[("height", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QueueScheduledError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the swap queue.
pub async fn queue_swap(configuration: &configuration::Configuration, params: QueueSwapParams) -> Result<Vec<crate::models::MsgSwap>, Error<QueueSwapError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/queue/swap", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = height {
        local_var_req_builder = local_var_req_builder.query(&[("height", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QueueSwapError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

