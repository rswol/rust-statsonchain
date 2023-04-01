# \SqlApi

All URIs are relative to *https://api.stacksdata.info*

Method | HTTP request | Description
------------- | ------------- | -------------
[**run_portfolio_tx_history**](SqlApi.md#run_portfolio_tx_history) | **GET** /v1/owner/{address}/portfolio | 
[**run_sql**](SqlApi.md#run_sql) | **POST** /v1/sql | 
[**run_sql_v2**](SqlApi.md#run_sql_v2) | **POST** /v1/sql-v2 | 
[**run_tx_history**](SqlApi.md#run_tx_history) | **GET** /v1/owner/{address}/history | 



## run_portfolio_tx_history

> crate::models::PaginationResponsePortfolioEntry run_portfolio_tx_history(address, next, limit)


Portfolio history for owner in reverse order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**next** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginationResponsePortfolioEntry**](PaginationResponsePortfolioEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_sql

> crate::models::Dataset run_sql(body)


Executes a provided SQL. See SQL API guide for details. https://stacksdata.info/sql.html

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**String**> | SQL query |  |

### Return type

[**crate::models::Dataset**](Dataset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_sql_v2

> crate::models::Dataset run_sql_v2(run_sql_request)


Executes a provided SQL. See SQL API guide for details https://stacksdata.info/sql.html.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_sql_request** | Option<[**RunSqlRequest**](RunSqlRequest.md)> |  |  |

### Return type

[**crate::models::Dataset**](Dataset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_tx_history

> crate::models::PaginationResponseTaxmanEntry run_tx_history(address, next, limit)


Transaction history for owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**next** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginationResponseTaxmanEntry**](PaginationResponseTaxmanEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

