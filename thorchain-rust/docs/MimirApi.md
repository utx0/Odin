# \MimirApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mimir**](MimirApi.md#mimir) | **GET** /thorchain/mimir | 
[**mimir_admin**](MimirApi.md#mimir_admin) | **GET** /thorchain/mimir/admin | 
[**mimir_key**](MimirApi.md#mimir_key) | **GET** /thorchain/mimir/key/{key} | 
[**mimir_node**](MimirApi.md#mimir_node) | **GET** /thorchain/mimir/node/{address} | 
[**mimir_nodes**](MimirApi.md#mimir_nodes) | **GET** /thorchain/mimir/nodes_all | 



## mimir

> ::std::collections::HashMap<String, String> mimir(height)


Returns current active mimir configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mimir_admin

> ::std::collections::HashMap<String, String> mimir_admin(height)


Returns current admin mimir configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mimir_key

> i64 mimir_key(key, height)


Returns current active mimir configuration for the provided key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | the mimir key to lookup | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mimir_node

> ::std::collections::HashMap<String, String> mimir_node(address, height)


Returns current node mimir configuration for the provided node address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mimir_nodes

> crate::models::MimirNodesResponse mimir_nodes(height)


Returns current node mimir votes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |

### Return type

[**crate::models::MimirNodesResponse**](MimirNodesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

