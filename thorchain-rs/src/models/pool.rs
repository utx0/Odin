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
pub struct Pool {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i64>,
    #[serde(rename = "pending_inbound_asset")]
    pub pending_inbound_asset: String,
    #[serde(rename = "pending_inbound_rune")]
    pub pending_inbound_rune: String,
    #[serde(rename = "balance_asset")]
    pub balance_asset: String,
    #[serde(rename = "balance_rune")]
    pub balance_rune: String,
    /// the total pool units, this is the sum of LP and synth units
    #[serde(rename = "pool_units")]
    pub pool_units: String,
    /// the total pool liquidity provider units
    #[serde(rename = "LP_units")]
    pub lp_units: String,
    /// the total synth units in the pool
    #[serde(rename = "synth_units")]
    pub synth_units: String,
    /// the total supply of synths for the asset
    #[serde(rename = "synth_supply")]
    pub synth_supply: String,
    /// the balance of L1 asset deposited into the Savers Vault
    #[serde(rename = "savers_depth")]
    pub savers_depth: String,
    /// the number of units owned by Savers
    #[serde(rename = "savers_units")]
    pub savers_units: String,
    /// whether additional synths cannot be minted
    #[serde(rename = "synth_mint_paused")]
    pub synth_mint_paused: bool,
    /// the amount of synth supply remaining before the current max supply is reached
    #[serde(rename = "synth_supply_remaining")]
    pub synth_supply_remaining: String,
    /// the amount of collateral collects for loans
    #[serde(rename = "loan_collateral")]
    pub loan_collateral: String,
    /// the current loan collateralization ratio
    #[serde(rename = "loan_cr")]
    pub loan_cr: String,
}

impl Pool {
    pub fn new(asset: String, status: String, pending_inbound_asset: String, pending_inbound_rune: String, balance_asset: String, balance_rune: String, pool_units: String, lp_units: String, synth_units: String, synth_supply: String, savers_depth: String, savers_units: String, synth_mint_paused: bool, synth_supply_remaining: String, loan_collateral: String, loan_cr: String) -> Pool {
        Pool {
            asset,
            status,
            decimals: None,
            pending_inbound_asset,
            pending_inbound_rune,
            balance_asset,
            balance_rune,
            pool_units,
            lp_units,
            synth_units,
            synth_supply,
            savers_depth,
            savers_units,
            synth_mint_paused,
            synth_supply_remaining,
            loan_collateral,
            loan_cr,
        }
    }
}


