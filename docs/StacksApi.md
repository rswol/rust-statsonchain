# \StacksApi

All URIs are relative to *https://api.stacksdata.info*

Method | HTTP request | Description
------------- | ------------- | -------------
[**arbitrage_mia**](StacksApi.md#arbitrage_mia) | **GET** /v1/mia-arb | 
[**arbitrage_nyc**](StacksApi.md#arbitrage_nyc) | **GET** /v1/nyc-arb | 
[**arkadiko_vaults**](StacksApi.md#arkadiko_vaults) | **GET** /v1/owner/{address}/arkadiko-vaults | 
[**block**](StacksApi.md#block) | **GET** /v1/block | 
[**coins**](StacksApi.md#coins) | **GET** /v1/coins | 
[**deserialize**](StacksApi.md#deserialize) | **GET** /v1/deserialize/{value} | 
[**price**](StacksApi.md#price) | **GET** /v1/price | 
[**run**](StacksApi.md#run) | **POST** /v1/run | 
[**ts**](StacksApi.md#ts) | **POST** /v1/ts | 



## arbitrage_mia

> Vec<crate::models::Arbitrage> arbitrage_mia()


MIA citycoin mining arbitrage.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Arbitrage>**](Arbitrage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## arbitrage_nyc

> Vec<crate::models::Arbitrage> arbitrage_nyc()


NYC citycoin mining arbitrage.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Arbitrage>**](Arbitrage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## arkadiko_vaults

> arkadiko_vaults(address)


Deserialize serialized Clarity values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block

> crate::models::BlockSpec block()


Returns the latest block number.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockSpec**](BlockSpec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## coins

> Vec<crate::models::CoinInfo> coins()


Coin contracts holders and supply.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CoinInfo>**](CoinInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deserialize

> deserialize(value)


Deserialize serialized Clarity values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## price

> Vec<crate::models::PriceSpec> price(symbol, from, to)


Fetched latest prices for corresponding symbol.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Can be STX-USD, MIA-USD, or BTC-USD |  |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PriceSpec>**](PriceSpec.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run

> crate::models::Dataset run(run_request)


Runs a predefined report, the report might use provided block. If block is not specified the latest one is assumed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_request** | Option<[**RunRequest**](RunRequest.md)> | reportName must be one of active_contracts, active_recent_contracts, smart_contracts, smart_contracts_total, pools, wallets, wallets_dist, whales, nft_contracts. |  |

### Return type

[**crate::models::Dataset**](Dataset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ts

> crate::models::Dataset ts(run_request)


Runs a predefined time-series report (e.g. price feed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_request** | Option<[**RunRequest**](RunRequest.md)> |  |  |

### Return type

[**crate::models::Dataset**](Dataset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

