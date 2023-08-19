# QuoteSaverDepositResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inbound_address** | **String** | the inbound address for the transaction on the source chain | 
**inbound_confirmation_blocks** | Option<**i64**> | the approximate number of source chain blocks required before processing | [optional]
**inbound_confirmation_seconds** | Option<**i64**> | the approximate seconds for block confirmations required before processing | [optional]
**outbound_delay_blocks** | Option<**i64**> | the number of thorchain blocks the outbound will be delayed | [optional]
**outbound_delay_seconds** | Option<**i64**> | the approximate seconds for the outbound delay before it will be sent | [optional]
**fees** | [**crate::models::QuoteFees**](QuoteFees.md) |  | 
**slippage_bps** | **i64** | the total swap slippage in basis points | 
**streaming_slippage_bps** | Option<**i64**> | the total streaming swap slippage in basis points | [optional]
**router** | Option<**String**> | the EVM chain router contract address | [optional]
**expiry** | **i64** | expiration timestamp in unix seconds | 
**warning** | **String** | static warning message | 
**notes** | **String** | chain specific quote notes | 
**dust_threshold** | Option<**String**> | Defines the minimum transaction size for the chain in base units (sats, wei, uatom). Transctions with asset amounts lower than the dust_threshold are ignored. | [optional]
**recommended_min_amount_in** | Option<**String**> | The recommended minimum inbound amount for this transaction type & inbound asset. Sending less than this amount could result in failed refunds. | [optional]
**memo** | **String** | generated memo for the deposit | 
**expected_amount_out** | Option<**String**> | same as expected_amount_deposit, to be deprecated in favour of expected_amount_deposit | [optional]
**expected_amount_deposit** | **String** | the amount of the target asset the user can expect to deposit after fees | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


