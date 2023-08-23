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

/// struct for passing parameters to the method [`pool`]
#[derive(Clone, Debug, Default)]
pub struct PoolParams {
    pub asset: String,
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`pools`]
#[derive(Clone, Debug, Default)]
pub struct PoolsParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}


/// struct for typed errors of method [`pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PoolError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PoolsError {
    UnknownValue(serde_json::Value),
}


/// Returns the pool information for the provided asset.
pub async fn pool(configuration: &configuration::Configuration, params: PoolParams) -> Result<crate::models::Pool, Error<PoolError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let asset = params.asset;
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/pool/{asset}", local_var_configuration.base_path, asset=crate::apis::urlencode(asset));
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
        let local_var_entity: Option<PoolError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the pool information for all assets.
pub async fn pools(configuration: &configuration::Configuration, params: PoolsParams) -> Result<Vec<crate::models::Pool>, Error<PoolsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/pools", local_var_configuration.base_path);
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
        let local_var_entity: Option<PoolsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
