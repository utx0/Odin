# ObservedTx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx** | [**crate::models::Tx**](Tx.md) |  | 
**status** | Option<**String**> |  | [optional]
**out_hashes** | Option<**Vec<String>**> |  | [optional]
**block_height** | Option<**i64**> | same as external_observed_height, to be deprecated in favour of external_observed_height | [optional]
**external_observed_height** | Option<**i64**> | the block height on the external source chain when the transaction was observed, not provided if chain is THOR | [optional]
**signers** | Option<**Vec<String>**> |  | [optional]
**observed_pub_key** | Option<**String**> |  | [optional]
**keysign_ms** | Option<**i64**> |  | [optional]
**finalise_height** | Option<**i64**> | same as external_confirmation_delay_height, to be deprecated in favour of external_confirmation_delay_height | [optional]
**external_confirmation_delay_height** | Option<**i64**> | the block height on the external source chain when confirmation counting will be complete, not provided if chain is THOR | [optional]
**aggregator** | Option<**String**> | the outbound aggregator to use, will also match a suffix | [optional]
**aggregator_target** | Option<**String**> | the aggregator target asset provided to transferOutAndCall | [optional]
**aggregator_target_limit** | Option<**String**> | the aggregator target asset limit provided to transferOutAndCall | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


