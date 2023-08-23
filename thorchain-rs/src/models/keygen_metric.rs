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
pub struct KeygenMetric {
    #[serde(rename = "pub_key", skip_serializing_if = "Option::is_none")]
    pub pub_key: Option<String>,
    #[serde(rename = "node_tss_times")]
    pub node_tss_times: Vec<crate::models::NodeKeygenMetric>,
}

impl KeygenMetric {
    pub fn new(node_tss_times: Vec<crate::models::NodeKeygenMetric>) -> KeygenMetric {
        KeygenMetric {
            pub_key: None,
            node_tss_times,
        }
    }
}

