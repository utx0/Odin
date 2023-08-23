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
pub struct VaultInfo {
    #[serde(rename = "pub_key")]
    pub pub_key: String,
    #[serde(rename = "routers")]
    pub routers: Vec<crate::models::VaultRouter>,
}

impl VaultInfo {
    pub fn new(pub_key: String, routers: Vec<crate::models::VaultRouter>) -> VaultInfo {
        VaultInfo {
            pub_key,
            routers,
        }
    }
}

