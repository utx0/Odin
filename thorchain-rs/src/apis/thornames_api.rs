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

/// struct for passing parameters to the method [`thorname`]
#[derive(Clone, Debug, Default)]
pub struct ThornameParams {
    /// the thornode to lookup
    pub name: String,
    /// optional block height, defaults to current tip
    pub height: Option<i64>
}


/// struct for typed errors of method [`thorname`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThornameError {
    UnknownValue(serde_json::Value),
}


/// Returns addresses registered to the provided thorname.
pub async fn thorname(configuration: &configuration::Configuration, params: ThornameParams) -> Result<Vec<crate::models::Thorname>, Error<ThornameError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let height = params.height;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/thorchain/thorname/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
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
        let local_var_entity: Option<ThornameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

