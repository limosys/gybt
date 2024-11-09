# \SolanaRpcApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_rate_limit**](SolanaRpcApi.md#api_get_rate_limit) | **GET** /api/v2/rate-limit | Returns the details of an account rate limits
[**api_get_transaction**](SolanaRpcApi.md#api_get_transaction) | **GET** /api/v2/transaction | Returns the details of a transaction



## api_get_rate_limit

> models::ApiGetRateLimitResponse api_get_rate_limit()
Returns the details of an account rate limits

Returns the details of an account rate limits

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiGetRateLimitResponse**](apiGetRateLimitResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_transaction

> models::ApiGetTransactionResponse api_get_transaction(signature)
Returns the details of a transaction

Returns the details of a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signature** | Option<**String**> |  |  |

### Return type

[**models::ApiGetTransactionResponse**](apiGetTransactionResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

