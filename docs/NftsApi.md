# \NftsApi

All URIs are relative to *https://api.stacksdata.info*

Method | HTTP request | Description
------------- | ------------- | -------------
[**address_history**](NftsApi.md#address_history) | **GET** /nft/address/{address}/history | 
[**assets**](NftsApi.md#assets) | **GET** /nft/assets/{recipient} | 
[**contracts**](NftsApi.md#contracts) | **GET** /nft/contracts | 
[**floor**](NftsApi.md#floor) | **GET** /nft/contracts/{contract}/floor | 
[**owners**](NftsApi.md#owners) | **GET** /nft/contracts/{contract}/owners | 
[**prices**](NftsApi.md#prices) | **GET** /nft/contracts/{contract}/price | 
[**token**](NftsApi.md#token) | **GET** /nft/contracts/{contract}/tokens/{token} | 
[**tokens**](NftsApi.md#tokens) | **GET** /nft/contracts/{contract}/tokens | 
[**total_volumes**](NftsApi.md#total_volumes) | **GET** /nft/volume | 
[**transactions**](NftsApi.md#transactions) | **GET** /nft/contracts/{contract}/transactions | 
[**volumes**](NftsApi.md#volumes) | **GET** /nft/contracts/{contract}/volume | 



## address_history

> Vec<crate::models::NftHistory> address_history(address)


NFT events history for a specific address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NftHistory>**](NftHistory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assets

> Vec<crate::models::NftHolding> assets(recipient)


NFT assets with corresponding transactions by recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NftHolding>**](NftHolding.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts

> Vec<crate::models::NftContract> contracts()


Returns a list of all NFT contracts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NftContract>**](NftContract.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## floor

> Vec<crate::models::NftFloor> floor(contract, timeframe)


Floor price for given contract grouped by timeframe (1h, 15m)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |
**timeframe** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::NftFloor>**](NftFloor.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## owners

> Vec<crate::models::NftOwner> owners(contract)


Returns a list of all owner of given NFT contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NftOwner>**](NftOwner.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prices

> Vec<crate::models::NftPrice> prices(contract, datepart)


Returns average prices per time period for given NFT contract in the last 60 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |
**datepart** | **String** | Can be hour, day, or week | [required] |

### Return type

[**Vec<crate::models::NftPrice>**](NftPrice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token

> crate::models::NftToken token(contract, token)


Returns a token information for given NFT contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |
**token** | **i64** |  | [required] |

### Return type

[**crate::models::NftToken**](NftToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens

> Vec<crate::models::NftToken> tokens(contract)


Returns a list of all tokens of given NFT contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NftToken>**](NftToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_volumes

> Vec<crate::models::NftVolume> total_volumes(days)


Returns volumes per day for all NFT contracts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::NftVolume>**](NftVolume.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions

> Vec<crate::models::NftTransactions> transactions(contract)


Returns number of transactions per day for given NFT contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NftTransactions>**](NftTransactions.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes

> Vec<crate::models::NftVolume> volumes(contract)


Returns volumes per day for given NFT contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | **String** |  | [required] |

### Return type

[**Vec<crate::models::NftVolume>**](NftVolume.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

