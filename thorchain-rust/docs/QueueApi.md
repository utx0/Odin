# \QueueApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**queue**](QueueApi.md#queue) | **GET** /thorchain/queue | 
[**queue_outbound**](QueueApi.md#queue_outbound) | **GET** /thorchain/queue/outbound | 
[**queue_scheduled**](QueueApi.md#queue_scheduled) | **GET** /thorchain/queue/scheduled | 
[**queue_swap**](QueueApi.md#queue_swap) | **GET** /thorchain/queue/swap | 



## queue

> crate::models::QueueResponse queue(height)


Returns queue statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::QueueResponse**](QueueResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue_outbound

> Vec<crate::models::TxOutItem> queue_outbound(height)


Returns the outbound queue including estimated RUNE values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::TxOutItem>**](TxOutItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue_scheduled

> Vec<crate::models::TxOutItem> queue_scheduled(height)


Returns the scheduled queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::TxOutItem>**](TxOutItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue_swap

> Vec<crate::models::MsgSwap> queue_swap(height)


Returns the swap queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::MsgSwap>**](MsgSwap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

