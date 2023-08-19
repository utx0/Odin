# MsgSwap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx** | [**crate::models::Tx**](Tx.md) |  | 
**target_asset** | **String** | the asset to be swapped to | 
**destination** | Option<**String**> | the destination address to receive the swap output | [optional]
**trade_target** | **String** | the minimum amount of output asset to receive (else cancelling and refunding the swap) | 
**affiliate_address** | Option<**String**> | the affiliate address which will receive any affiliate fee | [optional]
**affiliate_basis_points** | **String** | the affiliate fee in basis points | 
**signer** | Option<**String**> | the signer (sender) of the transaction | [optional]
**aggregator** | Option<**String**> | the contract address if an aggregator is specified for a non-THORChain SwapOut | [optional]
**aggregator_target_address** | Option<**String**> | the desired output asset of the aggregator SwapOut | [optional]
**aggregator_target_limit** | Option<**String**> | the minimum amount of SwapOut asset to receive (else cancelling the SwapOut and receiving THORChain's output) | [optional]
**order_type** | Option<**i64**> | 0 if a market order (immediately completed or refunded), 1 if a limit order (held until fulfillable) | [optional]
**stream_quantity** | Option<**i32**> | number of swaps to execute in a streaming swap | [optional]
**stream_interval** | Option<**i32**> | the interval (in blocks) to execute the streaming swap | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


