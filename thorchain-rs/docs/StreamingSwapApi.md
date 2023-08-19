# \StreamingSwapApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stream_swap**](StreamingSwapApi.md#stream_swap) | **GET** /thorchain/swap/streaming/{hash} | 
[**stream_swaps**](StreamingSwapApi.md#stream_swaps) | **GET** /thorchain/swaps/streaming | 



## stream_swap

> crate::models::StreamingSwap stream_swap(hash, height)


Returns the state of a streaming swap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::StreamingSwap**](StreamingSwap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_swaps

> Vec<crate::models::StreamingSwap> stream_swaps(height)


Returns the state of all streaming swaps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::StreamingSwap>**](StreamingSwap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

