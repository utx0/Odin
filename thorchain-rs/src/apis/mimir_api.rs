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

/// struct for passing parameters to the method [`mimir`]
#[derive(Clone, Debug, Default)]
pub struct MimirParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`mimir_admin`]
#[derive(Clone, Debug, Default)]
pub struct MimirAdminParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`mimir_key`]
#[derive(Clone, Debug, Default)]
pub struct MimirKeyParams {
    /// the mimir key to lookup
    pub key: String,
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`mimir_node`]
#[derive(Clone, Debug, Default)]
pub struct MimirNodeParams {
    pub address: String,
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}

/// struct for passing parameters to the method [`mimir_nodes`]
#[derive(Clone, Debug, Default)]
pub struct MimirNodesParams {
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}


/// struct for typed errors of method [`mimir`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MimirError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mimir_admin`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MimirAdminError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mimir_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MimirKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mimir_node`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MimirNodeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mimir_nodes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MimirNodesError {
    UnknownValue(serde_json::Value),
}


/// Returns current active mimir configuration.
pub async fn mimir(configuration: &configuration::Configuration, params: MimirParams) -> Result<::std::collections::HashMap<String, String>, Error<MimirError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/mimir", local_var_configuration.base_path);
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
        let local_var_entity: Option<MimirError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns current admin mimir configuration.
pub async fn mimir_admin(configuration: &configuration::Configuration, params: MimirAdminParams) -> Result<::std::collections::HashMap<String, String>, Error<MimirAdminError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/mimir/admin", local_var_configuration.base_path);
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
        let local_var_entity: Option<MimirAdminError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns current active mimir configuration for the provided key.
pub async fn mimir_key(configuration: &configuration::Configuration, params: MimirKeyParams) -> Result<i64, Error<MimirKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let key = params.key;
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/mimir/key/{key}", local_var_configuration.base_path, key=crate::apis::urlencode(key));
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
        let local_var_entity: Option<MimirKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns current node mimir configuration for the provided node address.
pub async fn mimir_node(configuration: &configuration::Configuration, params: MimirNodeParams) -> Result<::std::collections::HashMap<String, String>, Error<MimirNodeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let address = params.address;
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/mimir/node/{address}", local_var_configuration.base_path, address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<MimirNodeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns current node mimir votes.
pub async fn mimir_nodes(configuration: &configuration::Configuration, params: MimirNodesParams) -> Result<crate::models::MimirNodesResponse, Error<MimirNodesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/mimir/nodes_all", local_var_configuration.base_path);
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
        let local_var_entity: Option<MimirNodesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

