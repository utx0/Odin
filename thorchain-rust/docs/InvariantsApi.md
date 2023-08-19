# \InvariantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invariant**](InvariantsApi.md#invariant) | **GET** /thorchain/invariant/{invariant} | 
[**invariants**](InvariantsApi.md#invariants) | **GET** /thorchain/invariants | 



## invariant

> crate::models::InvariantResponse invariant(invariant, height)


Returns result of running the given invariant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invariant** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::InvariantResponse**](InvariantResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invariants

> crate::models::InvariantsResponse invariants(height)


Returns a list of available invariants.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::InvariantsResponse**](InvariantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

