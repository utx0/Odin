# BaseQuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inbound_address** | Option<**String**> | the inbound address for the transaction on the source chain | [optional]
**inbound_confirmation_blocks** | Option<**i64**> | the approximate number of source chain blocks required before processing | [optional]
**inbound_confirmation_seconds** | Option<**i64**> | the approximate seconds for block confirmations required before processing | [optional]
**outbound_delay_blocks** | Option<**i64**> | the number of thorchain blocks the outbound will be delayed | [optional]
**outbound_delay_seconds** | Option<**i64**> | the approximate seconds for the outbound delay before it will be sent | [optional]
**fees** | Option<[**crate::models::QuoteFees**](QuoteFees.md)> |  | [optional]
**slippage_bps** | Option<**i64**> | the total swap slippage in basis points | [optional]
**streaming_slippage_bps** | Option<**i64**> | the total streaming swap slippage in basis points | [optional]
**router** | Option<**String**> | the EVM chain router contract address | [optional]
**expiry** | Option<**i64**> | expiration timestamp in unix seconds | [optional]
**warning** | Option<**String**> | static warning message | [optional]
**notes** | Option<**String**> | chain specific quote notes | [optional]
**dust_threshold** | Option<**String**> | Defines the minimum transaction size for the chain in base units (sats, wei, uatom). Transctions with asset amounts lower than the dust_threshold are ignored. | [optional]
**recommended_min_amount_in** | Option<**String**> | The recommended minimum inbound amount for this transaction type & inbound asset. Sending less than this amount could result in failed refunds. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


