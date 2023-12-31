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
pub struct Tx {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(rename = "from_address", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "to_address", skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    #[serde(rename = "coins")]
    pub coins: Vec<crate::models::Coin>,
    #[serde(rename = "gas")]
    pub gas: Vec<crate::models::Coin>,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl Tx {
    pub fn new(coins: Vec<crate::models::Coin>, gas: Vec<crate::models::Coin>) -> Tx {
        Tx {
            id: None,
            chain: None,
            from_address: None,
            to_address: None,
            coins,
            gas,
            memo: None,
        }
    }
}


