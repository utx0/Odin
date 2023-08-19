# TxDetailsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_id** | Option<**String**> |  | [optional]
**tx** | [**crate::models::ObservedTx**](ObservedTx.md) |  | 
**txs** | [**Vec<crate::models::ObservedTx>**](ObservedTx.md) |  | 
**actions** | [**Vec<crate::models::TxOutItem>**](TxOutItem.md) |  | 
**out_txs** | [**Vec<crate::models::Tx>**](Tx.md) |  | 
**consensus_height** | Option<**i64**> | the thorchain height at which the inbound reached consensus | [optional]
**finalised_height** | Option<**i64**> | the thorchain height at which the outbound was finalised | [optional]
**updated_vault** | Option<**bool**> |  | [optional]
**reverted** | Option<**bool**> |  | [optional]
**outbound_height** | Option<**i64**> | the thorchain height for which the outbound was scheduled | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


