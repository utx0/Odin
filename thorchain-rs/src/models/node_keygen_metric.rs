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
pub struct NodeKeygenMetric {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "tss_time", skip_serializing_if = "Option::is_none")]
    pub tss_time: Option<String>,
}

impl NodeKeygenMetric {
    pub fn new() -> NodeKeygenMetric {
        NodeKeygenMetric {
            address: None,
            tss_time: None,
        }
    }
}


