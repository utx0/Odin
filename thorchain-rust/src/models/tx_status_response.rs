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
pub struct TxStatusResponse {
    #[serde(rename = "tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<Box<crate::models::Tx>>,
    #[serde(rename = "planned_out_txs", skip_serializing_if = "Option::is_none")]
    pub planned_out_txs: Option<Vec<crate::models::TxStatusResponsePlannedOutTxsInner>>,
    #[serde(rename = "out_txs", skip_serializing_if = "Option::is_none")]
    pub out_txs: Option<Vec<crate::models::Tx>>,
    #[serde(rename = "stages")]
    pub stages: Box<crate::models::TxStagesResponse>,
}

impl TxStatusResponse {
    pub fn new(stages: crate::models::TxStagesResponse) -> TxStatusResponse {
        TxStatusResponse {
            tx: None,
            planned_out_txs: None,
            out_txs: None,
            stages: Box::new(stages),
        }
    }
}

