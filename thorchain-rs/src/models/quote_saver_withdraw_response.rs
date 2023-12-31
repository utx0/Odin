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
pub struct QuoteSaverWithdrawResponse {
    /// the inbound address for the transaction on the source chain
    #[serde(rename = "inbound_address")]
    pub inbound_address: String,
    /// the approximate number of source chain blocks required before processing
    #[serde(rename = "inbound_confirmation_blocks", skip_serializing_if = "Option::is_none")]
    pub inbound_confirmation_blocks: Option<i64>,
    /// the approximate seconds for block confirmations required before processing
    #[serde(rename = "inbound_confirmation_seconds", skip_serializing_if = "Option::is_none")]
    pub inbound_confirmation_seconds: Option<i64>,
    /// the number of thorchain blocks the outbound will be delayed
    #[serde(rename = "outbound_delay_blocks")]
    pub outbound_delay_blocks: i64,
    /// the approximate seconds for the outbound delay before it will be sent
    #[serde(rename = "outbound_delay_seconds")]
    pub outbound_delay_seconds: i64,
    #[serde(rename = "fees")]
    pub fees: Box<crate::models::QuoteFees>,
    /// the total swap slippage in basis points
    #[serde(rename = "slippage_bps")]
    pub slippage_bps: i64,
    /// the total streaming swap slippage in basis points
    #[serde(rename = "streaming_slippage_bps", skip_serializing_if = "Option::is_none")]
    pub streaming_slippage_bps: Option<i64>,
    /// the EVM chain router contract address
    #[serde(rename = "router", skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    /// expiration timestamp in unix seconds
    #[serde(rename = "expiry")]
    pub expiry: i64,
    /// static warning message
    #[serde(rename = "warning")]
    pub warning: String,
    /// chain specific quote notes
    #[serde(rename = "notes")]
    pub notes: String,
    /// Defines the minimum transaction size for the chain in base units (sats, wei, uatom). Transctions with asset amounts lower than the dust_threshold are ignored.
    #[serde(rename = "dust_threshold", skip_serializing_if = "Option::is_none")]
    pub dust_threshold: Option<String>,
    /// The recommended minimum inbound amount for this transaction type & inbound asset. Sending less than this amount could result in failed refunds.
    #[serde(rename = "recommended_min_amount_in", skip_serializing_if = "Option::is_none")]
    pub recommended_min_amount_in: Option<String>,
    /// generated memo for the withdraw, the client can use this OR send the dust amount
    #[serde(rename = "memo")]
    pub memo: String,
    /// the dust amount of the target asset the user should send to initialize the withdraw, the client can send this OR provide the memo
    #[serde(rename = "dust_amount")]
    pub dust_amount: String,
    /// the amount of the target asset the user can expect to withdraw after fees in 1e8 decimals
    #[serde(rename = "expected_amount_out")]
    pub expected_amount_out: String,
}

impl QuoteSaverWithdrawResponse {
    pub fn new(inbound_address: String, outbound_delay_blocks: i64, outbound_delay_seconds: i64, fees: crate::models::QuoteFees, slippage_bps: i64, expiry: i64, warning: String, notes: String, memo: String, dust_amount: String, expected_amount_out: String) -> QuoteSaverWithdrawResponse {
        QuoteSaverWithdrawResponse {
            inbound_address,
            inbound_confirmation_blocks: None,
            inbound_confirmation_seconds: None,
            outbound_delay_blocks,
            outbound_delay_seconds,
            fees: Box::new(fees),
            slippage_bps,
            streaming_slippage_bps: None,
            router: None,
            expiry,
            warning,
            notes,
            dust_threshold: None,
            recommended_min_amount_in: None,
            memo,
            dust_amount,
            expected_amount_out,
        }
    }
}


