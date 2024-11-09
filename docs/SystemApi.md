# \SystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_priority_fee**](SystemApi.md#api_get_priority_fee) | **GET** /api/v2/system/priority-fee | Estimated priority fee
[**api_get_priority_fee_by_program**](SystemApi.md#api_get_priority_fee_by_program) | **GET** /api/v2/system/priority-fee-by-program | Estimated priority fee
[**api_get_recent_block_hash**](SystemApi.md#api_get_recent_block_hash) | **GET** /api/v1/system/blockhash | Recent block hash
[**api_get_recent_block_hash_v2**](SystemApi.md#api_get_recent_block_hash_v2) | **GET** /api/v2/system/blockhash | Recent block hash
[**api_get_server_time**](SystemApi.md#api_get_server_time) | **GET** /api/v1/system/time | Server time



## api_get_priority_fee

> models::ApiGetPriorityFeeResponse api_get_priority_fee(project, percentile)
Estimated priority fee

Returns an estimated prioritization fee based on recent transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> |  |  |[default to P_UNKNOWN]
**percentile** | Option<**f64**> |  |  |

### Return type

[**models::ApiGetPriorityFeeResponse**](apiGetPriorityFeeResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_priority_fee_by_program

> models::ApiGetPriorityFeeByProgramResponse api_get_priority_fee_by_program(programs)
Estimated priority fee

Returns an estimated prioritization fee based on recent transactions. supports filtering by program ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**programs** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::ApiGetPriorityFeeByProgramResponse**](apiGetPriorityFeeByProgramResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_recent_block_hash

> models::ApiGetRecentBlockHashResponse api_get_recent_block_hash()
Recent block hash

Returns recent block hash

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiGetRecentBlockHashResponse**](apiGetRecentBlockHashResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_recent_block_hash_v2

> models::ApiGetRecentBlockHashResponseV2 api_get_recent_block_hash_v2(offset)
Recent block hash

Returns recent block hash, supports optional offset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> |  |  |

### Return type

[**models::ApiGetRecentBlockHashResponseV2**](apiGetRecentBlockHashResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_server_time

> models::ApiGetServerTimeResponse api_get_server_time()
Server time

Returns current time on the server

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiGetServerTimeResponse**](apiGetServerTimeResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

