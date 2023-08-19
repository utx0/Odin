# \LiquidityProvidersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**liquidity_provider**](LiquidityProvidersApi.md#liquidity_provider) | **GET** /thorchain/pool/{asset}/liquidity_provider/{address} | 
[**liquidity_providers**](LiquidityProvidersApi.md#liquidity_providers) | **GET** /thorchain/pool/{asset}/liquidity_providers | 



## liquidity_provider

> crate::models::LiquidityProvider liquidity_provider(asset, address, height)


Returns the liquidity provider information for an address and asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**address** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::LiquidityProvider**](LiquidityProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## liquidity_providers

> Vec<crate::models::LiquidityProviderSummary> liquidity_providers(asset, height)


Returns all liquidity provider information for an asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::LiquidityProviderSummary>**](LiquidityProviderSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

