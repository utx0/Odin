# \NetworkApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ban**](NetworkApi.md#ban) | **GET** /thorchain/ban/{address} | 
[**constants**](NetworkApi.md#constants) | **GET** /thorchain/constants | 
[**inbound_addresses**](NetworkApi.md#inbound_addresses) | **GET** /thorchain/inbound_addresses | 
[**lastblock**](NetworkApi.md#lastblock) | **GET** /thorchain/lastblock | 
[**lastblock_chain**](NetworkApi.md#lastblock_chain) | **GET** /thorchain/lastblock/{chain} | 
[**network**](NetworkApi.md#network) | **GET** /thorchain/network | 
[**ragnarok**](NetworkApi.md#ragnarok) | **GET** /thorchain/ragnarok | 
[**version**](NetworkApi.md#version) | **GET** /thorchain/version | 



## ban

> crate::models::BanResponse ban(address, height)


Returns the ban status for the provided node address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::BanResponse**](BanResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## constants

> crate::models::ConstantsResponse constants(height)


Returns constant configuration, can be overridden by mimir.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::ConstantsResponse**](ConstantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inbound_addresses

> Vec<crate::models::InboundAddress> inbound_addresses(height)


Returns the set of asgard addresses that should be used for inbound transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::InboundAddress>**](InboundAddress.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lastblock

> Vec<crate::models::LastBlock> lastblock(height)


Returns the last block information for all chains.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::LastBlock>**](LastBlock.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lastblock_chain

> Vec<crate::models::LastBlock> lastblock_chain(chain, height)


Returns the last block information for the provided chain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chain** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**Vec<crate::models::LastBlock>**](LastBlock.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network

> crate::models::NetworkResponse network(height)


Returns network overview statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::NetworkResponse**](NetworkResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ragnarok

> bool ragnarok(height)


Returns a boolean indicating whether the chain is in ragnarok.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version

> crate::models::VersionResponse version(height)


Returns the network's current THORNode version, the network's next THORNode version, and the querier's THORNode version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::VersionResponse**](VersionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

