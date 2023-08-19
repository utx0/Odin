# \TssApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**keysign**](TssApi.md#keysign) | **GET** /thorchain/keysign/{height} | 
[**keysign_pubkey**](TssApi.md#keysign_pubkey) | **GET** /thorchain/keysign/{height}/{pubkey} | 
[**metrics**](TssApi.md#metrics) | **GET** /thorchain/metrics | 
[**metrics_keygen**](TssApi.md#metrics_keygen) | **GET** /thorchain/metric/keygen/{pubkey} | 



## keysign

> crate::models::KeysignResponse keysign(height)


Returns keysign information for the provided height - the height being the first block a tx out item appears in the signed-but-unobserved outbound queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **i64** |  | [required] |

### Return type

[**crate::models::KeysignResponse**](KeysignResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## keysign_pubkey

> crate::models::KeysignResponse keysign_pubkey(height, pubkey)


Returns keysign information for the provided height and pubkey - the height being the block at which a tx out item is scheduled to be signed and moved from the scheduled outbound queue to the outbound queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **i64** |  | [required] |
**pubkey** | **String** |  | [required] |

### Return type

[**crate::models::KeysignResponse**](KeysignResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metrics

> crate::models::MetricsResponse metrics(height)


Returns keygen and keysign metrics for current vaults.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::MetricsResponse**](MetricsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metrics_keygen

> Vec<crate::models::KeygenMetric> metrics_keygen(pubkey, height)


Returns keygen metrics for the provided vault pubkey.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::KeygenMetric>**](KeygenMetric.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

