# \SaversApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**saver**](SaversApi.md#saver) | **GET** /thorchain/pool/{asset}/saver/{address} | 
[**savers**](SaversApi.md#savers) | **GET** /thorchain/pool/{asset}/savers | 



## saver

> crate::models::Saver saver(asset, address, height)


Returns the saver position given then savers pool and address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**address** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::Saver**](Saver.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## savers

> Vec<crate::models::Saver> savers(asset, height)


Returns all savers for the savers pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::Saver>**](Saver.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

