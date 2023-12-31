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
pub struct NodeBondProvider {
    #[serde(rename = "bond_address", skip_serializing_if = "Option::is_none")]
    pub bond_address: Option<String>,
    #[serde(rename = "bond", skip_serializing_if = "Option::is_none")]
    pub bond: Option<String>,
}

impl NodeBondProvider {
    pub fn new() -> NodeBondProvider {
        NodeBondProvider {
            bond_address: None,
            bond: None,
        }
    }
}


