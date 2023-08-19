# InboundAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chain** | Option<**String**> |  | [optional]
**pub_key** | Option<**String**> |  | [optional]
**address** | Option<**String**> |  | [optional]
**router** | Option<**String**> |  | [optional]
**halted** | **bool** | Returns true if trading is unavailable for this chain, either because trading is halted globally or specifically for this chain | 
**global_trading_paused** | Option<**bool**> | Returns true if trading is paused globally | [optional]
**chain_trading_paused** | Option<**bool**> | Returns true if trading is paused for this chain | [optional]
**chain_lp_actions_paused** | Option<**bool**> | Returns true if LP actions are paused for this chain | [optional]
**gas_rate** | Option<**String**> | The minimum fee rate used by vaults to send outbound TXs. The actual fee rate may be higher. For EVM chains this is returned in gwei (1e9). | [optional]
**gas_rate_units** | Option<**String**> | Units of the gas_rate. | [optional]
**outbound_tx_size** | Option<**String**> | Avg size of outbound TXs on each chain. For UTXO chains it may be larger than average, as it takes into account vault consolidation txs, which can have many vouts | [optional]
**outbound_fee** | Option<**String**> | The total outbound fee charged to the user for outbound txs in the gas asset of the chain. | [optional]
**dust_threshold** | Option<**String**> | Defines the minimum transaction size for the chain in base units (sats, wei, uatom). Transactions with asset amounts lower than the dust_threshold are ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


