# \QuoteApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**quoteloanclose**](QuoteApi.md#quoteloanclose) | **GET** /thorchain/quote/loan/close | 
[**quoteloanopen**](QuoteApi.md#quoteloanopen) | **GET** /thorchain/quote/loan/open | 
[**quotesaverdeposit**](QuoteApi.md#quotesaverdeposit) | **GET** /thorchain/quote/saver/deposit | 
[**quotesaverwithdraw**](QuoteApi.md#quotesaverwithdraw) | **GET** /thorchain/quote/saver/withdraw | 
[**quoteswap**](QuoteApi.md#quoteswap) | **GET** /thorchain/quote/swap | 



## quoteloanclose

> crate::models::QuoteLoanCloseResponse quoteloanclose(height, from_asset, amount, to_asset, loan_owner, min_out)


Provide a quote estimate for the provided loan close.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |
**from_asset** | Option<**String**> | the asset used to repay the loan |  |
**amount** | Option<**i64**> | the asset amount in 1e8 decimals |  |
**to_asset** | Option<**String**> | the collateral asset of the loan |  |
**loan_owner** | Option<**String**> | the owner of the loan collateral |  |
**min_out** | Option<**String**> | the minimum amount of the target asset to accept |  |

### Return type

[**crate::models::QuoteLoanCloseResponse**](QuoteLoanCloseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quoteloanopen

> crate::models::QuoteLoanOpenResponse quoteloanopen(height, from_asset, amount, to_asset, destination, min_out, affiliate_bps, affiliate)


Provide a quote estimate for the provided loan open.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |
**from_asset** | Option<**String**> | the collateral asset |  |
**amount** | Option<**i64**> | the collateral asset amount in 1e8 decimals |  |
**to_asset** | Option<**String**> | the target asset to receive (loan denominated in TOR regardless) |  |
**destination** | Option<**String**> | the destination address, required to generate memo |  |
**min_out** | Option<**String**> | the minimum amount of the target asset to accept |  |
**affiliate_bps** | Option<**i64**> | the affiliate fee in basis points |  |
**affiliate** | Option<**String**> | the affiliate (address or thorname) |  |

### Return type

[**crate::models::QuoteLoanOpenResponse**](QuoteLoanOpenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotesaverdeposit

> crate::models::QuoteSaverDepositResponse quotesaverdeposit(height, asset, amount)


Provide a quote estimate for the provided saver deposit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |
**asset** | Option<**String**> | the asset to deposit |  |
**amount** | Option<**i64**> | the source asset amount in 1e8 decimals |  |

### Return type

[**crate::models::QuoteSaverDepositResponse**](QuoteSaverDepositResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotesaverwithdraw

> crate::models::QuoteSaverWithdrawResponse quotesaverwithdraw(height, asset, address, withdraw_bps)


Provide a quote estimate for the provided saver withdraw.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |
**asset** | Option<**String**> | the asset to withdraw |  |
**address** | Option<**String**> | the address for the position |  |
**withdraw_bps** | Option<**i64**> | the basis points of the existing position to withdraw |  |

### Return type

[**crate::models::QuoteSaverWithdrawResponse**](QuoteSaverWithdrawResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quoteswap

> crate::models::QuoteSwapResponse quoteswap(height, from_asset, to_asset, amount, destination, streaming_interval, streaming_quantity, from_address, tolerance_bps, affiliate_bps, affiliate)


Provide a quote estimate for the provided swap.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | Option<**i64**> | optional block height, defaults to current tip |  |
**from_asset** | Option<**String**> | the source asset |  |
**to_asset** | Option<**String**> | the target asset |  |
**amount** | Option<**i64**> | the source asset amount in 1e8 decimals |  |
**destination** | Option<**String**> | the destination address, required to generate memo |  |
**streaming_interval** | Option<**i64**> | the interval in which streaming swaps are swapped |  |
**streaming_quantity** | Option<**i64**> | the quantity of swaps within a streaming swap |  |
**from_address** | Option<**String**> | the from address, required if the from asset is a synth |  |
**tolerance_bps** | Option<**i64**> | the maximum basis points from the current feeless swap price to set the limit in the generated memo |  |
**affiliate_bps** | Option<**i64**> | the affiliate fee in basis points |  |
**affiliate** | Option<**String**> | the affiliate (address or thorname) |  |

### Return type

[**crate::models::QuoteSwapResponse**](QuoteSwapResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

