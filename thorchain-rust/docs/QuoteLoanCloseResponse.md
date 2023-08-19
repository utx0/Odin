# QuoteLoanCloseResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inbound_address** | Option<**String**> | the inbound address for the transaction on the source chain | [optional]
**inbound_confirmation_blocks** | Option<**i64**> | the approximate number of source chain blocks required before processing | [optional]
**inbound_confirmation_seconds** | Option<**i64**> | the approximate seconds for block confirmations required before processing | [optional]
**outbound_delay_blocks** | **i64** | the number of thorchain blocks the outbound will be delayed | 
**outbound_delay_seconds** | **i64** | the approximate seconds for the outbound delay before it will be sent | 
**fees** | [**crate::models::QuoteFees**](QuoteFees.md) |  | 
**slippage_bps** | Option<**i64**> | the total swap slippage in basis points | [optional]
**streaming_slippage_bps** | Option<**i64**> | the total streaming swap slippage in basis points | [optional]
**router** | Option<**String**> | the EVM chain router contract address | [optional]
**expiry** | **i64** | expiration timestamp in unix seconds | 
**warning** | **String** | static warning message | 
**notes** | **String** | chain specific quote notes | 
**dust_threshold** | Option<**String**> | Defines the minimum transaction size for the chain in base units (sats, wei, uatom). Transctions with asset amounts lower than the dust_threshold are ignored. | [optional]
**recommended_min_amount_in** | Option<**String**> | The recommended minimum inbound amount for this transaction type & inbound asset. Sending less than this amount could result in failed refunds. | [optional]
**memo** | **String** | generated memo for the loan close | 
**expected_amount_out** | **String** | the amount of collateral asset the user can expect to receive after fees in 1e8 decimals | 
**expected_collateral_withdrawn** | **String** | the expected amount of collateral decrease on the loan | 
**expected_debt_repaid** | **String** | the expected amount of TOR debt decrease on the loan | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


