# StreamingSwap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tx_id** | Option<**String**> | the hash of a transaction | [optional]
**interval** | Option<**i32**> | how often each swap is made, in blocks | [optional]
**quantity** | Option<**i32**> | the total number of swaps in a streaming swaps | [optional]
**count** | Option<**i32**> | the amount of swap attempts so far | [optional]
**last_height** | Option<**i64**> | the block height of the latest swap | [optional]
**trade_target** | Option<**String**> | the total number of tokens the swapper wants to receive of the output asset | [optional]
**deposit** | Option<**String**> | the number of input tokens the swapper has deposited | [optional]
**r#in** | Option<**String**> | the amount of input tokens that have been swapped so far | [optional]
**out** | Option<**String**> | the amount of output tokens that have been swapped so far | [optional]
**failed_swaps** | Option<**Vec<i32>**> | the list of swap indexes that failed | [optional]
**failed_swap_reasons** | Option<**Vec<String>**> | the list of reasons that sub-swaps have failed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


