# \PoolsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pool**](PoolsApi.md#pool) | **GET** /thorchain/pool/{asset} | 
[**pools**](PoolsApi.md#pools) | **GET** /thorchain/pools | 



## pool

> crate::models::Pool pool(asset, height)


Returns the pool information for the provided asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::Pool**](Pool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pools

> Vec<crate::models::Pool> pools(height)


Returns the pool information for all assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::Pool>**](Pool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

