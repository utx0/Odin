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

/// struct for passing parameters to the method [`saver`]
#[derive(Clone, Debug, Default)]
pub struct SaverParams {
    pub asset: String,
    pub address: String,
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`savers`]
#[derive(Clone, Debug, Default)]
pub struct SaversParams {
    pub asset: String,
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}


/// struct for typed errors of method [`saver`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SaverError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`savers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SaversError {
    UnknownValue(serde_json::Value),
}


/// Returns the saver position given then savers pool and address.
pub async fn saver(configuration: &configuration::Configuration, params: SaverParams) -> Result<crate::models::Saver, Error<SaverError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let asset = params.asset;
    let address = params.address;
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/pool/{asset}/saver/{address}", local_var_configuration.base_path, asset=crate::apis::urlencode(asset), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<SaverError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all savers for the savers pool.
pub async fn savers(configuration: &configuration::Configuration, params: SaversParams) -> Result<Vec<crate::models::Saver>, Error<SaversError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let asset = params.asset;
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/pool/{asset}/savers", local_var_configuration.base_path, asset=crate::apis::urlencode(asset));
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
        let local_var_entity: Option<SaversError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
