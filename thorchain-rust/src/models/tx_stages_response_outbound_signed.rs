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
pub struct TxStagesResponseOutboundSigned {
    /// THORChain height for which the external outbound is scheduled
    #[serde(rename = "scheduled_outbound_height", skip_serializing_if = "Option::is_none")]
    pub scheduled_outbound_height: Option<i64>,
    /// THORChain blocks since the scheduled outbound height
    #[serde(rename = "blocks_since_scheduled", skip_serializing_if = "Option::is_none")]
    pub blocks_since_scheduled: Option<i64>,
    /// returns true if an external transaction has been signed and broadcast (and observed in its mempool)
    #[serde(rename = "completed")]
    pub completed: bool,
}

impl TxStagesResponseOutboundSigned {
    pub fn new(completed: bool) -> TxStagesResponseOutboundSigned {
        TxStagesResponseOutboundSigned {
            scheduled_outbound_height: None,
            blocks_since_scheduled: None,
            completed,
        }
    }
}


