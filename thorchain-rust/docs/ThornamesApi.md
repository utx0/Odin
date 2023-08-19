# \ThornamesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**thorname**](ThornamesApi.md#thorname) | **GET** /thorchain/thorname/{name} | 



## thorname

> Vec<crate::models::Thorname> thorname(name, height)


Returns addresses registered to the provided thorname.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | the thornode to lookup | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::Thorname>**](Thorname.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

