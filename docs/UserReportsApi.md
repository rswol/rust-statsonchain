# \UserReportsApi

All URIs are relative to *https://api.stacksdata.info*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dataset**](UserReportsApi.md#create_dataset) | **POST** /cp-v1/dataset | 
[**create_user_report**](UserReportsApi.md#create_user_report) | **POST** /cp-v1/user-report | 
[**delete_user_report**](UserReportsApi.md#delete_user_report) | **DELETE** /cp-v1/user-report/{name} | 
[**drop_user**](UserReportsApi.md#drop_user) | **DELETE** /cp-v1/user | 
[**get_user_report**](UserReportsApi.md#get_user_report) | **GET** /cp-v1/user-report/{name} | 
[**list_datasets**](UserReportsApi.md#list_datasets) | **GET** /cp-v1/dataset | 
[**list_user_reports**](UserReportsApi.md#list_user_reports) | **GET** /cp-v1/user-report | 
[**list_users**](UserReportsApi.md#list_users) | **GET** /cp-v1/user | 
[**register_user**](UserReportsApi.md#register_user) | **POST** /cp-v1/user | 
[**run_user_report**](UserReportsApi.md#run_user_report) | **POST** /cp-v1/run-user-report/{name} | 
[**upload**](UserReportsApi.md#upload) | **POST** /cp-v1/dataset/{name} | 



## create_dataset

> crate::models::DatasetInfo create_dataset(dataset_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dataset_request** | [**DatasetRequest**](DatasetRequest.md) |  | [required] |

### Return type

[**crate::models::DatasetInfo**](DatasetInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_report

> crate::models::UserReport create_user_report(user_report_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_report_request** | Option<[**UserReportRequest**](UserReportRequest.md)> |  |  |

### Return type

[**crate::models::UserReport**](UserReport.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_report

> delete_user_report(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drop_user

> crate::models::User drop_user(drop_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**drop_user_request** | Option<[**DropUserRequest**](DropUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_report

> crate::models::UserReport get_user_report(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::UserReport**](UserReport.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_datasets

> Vec<crate::models::DatasetInfo> list_datasets()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DatasetInfo>**](DatasetInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_reports

> Vec<crate::models::UserReportInfo> list_user_reports()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserReportInfo>**](UserReportInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> Vec<crate::models::User> list_users()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_user

> crate::models::User register_user(register_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_user_request** | Option<[**RegisterUserRequest**](RegisterUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_user_report

> crate::models::ApiResponseUserReportInfo run_user_report(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::ApiResponseUserReportInfo**](ApiResponseUserReportInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload

> i64 upload(name, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

**i64**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: text/csv
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

