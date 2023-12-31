/*
 * Thornode API
 *
 * Thornode REST API.
 *
 * The version of the OpenAPI document: 1.119.0
 * Contact: devs@thorchain.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BanResponse {
    #[serde(rename = "node_address", skip_serializing_if = "Option::is_none")]
    pub node_address: Option<String>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i64>,
    #[serde(rename = "signers", skip_serializing_if = "Option::is_none")]
    pub signers: Option<Vec<String>>,
}

impl BanResponse {
    pub fn new() -> BanResponse {
        BanResponse {
            node_address: None,
            block_height: None,
            signers: None,
        }
    }
}


