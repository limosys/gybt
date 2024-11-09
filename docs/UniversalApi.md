# \UniversalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_price_0**](UniversalApi.md#api_get_price_0) | **GET** /api/v1/market/price | Token prices
[**api_post_submit_0**](UniversalApi.md#api_post_submit_0) | **POST** /api/v1/trade/submit | Transaction submit
[**api_post_submit_batch_0**](UniversalApi.md#api_post_submit_batch_0) | **POST** /api/v1/trade/submit-batch | Transaction batch submit
[**api_post_submit_batch_v2_0**](UniversalApi.md#api_post_submit_batch_v2_0) | **POST** /api/v2/submit-batch | Transaction batch submit
[**api_post_submit_v2_0**](UniversalApi.md#api_post_submit_v2_0) | **POST** /api/v2/submit | Transaction submit



## api_get_price_0

> models::ApiGetPriceResponse api_get_price_0(tokens)
Token prices

Returns the list of prices for specified tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tokens** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::ApiGetPriceResponse**](apiGetPriceResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_submit_0

> models::ApiPostSubmitResponse api_post_submit_0(body)
Transaction submit

Submits a signed transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostSubmitRequest**](ApiPostSubmitRequest.md) |  | [required] |

### Return type

[**models::ApiPostSubmitResponse**](apiPostSubmitResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_submit_batch_0

> models::ApiPostSubmitBatchResponse api_post_submit_batch_0(body)
Transaction batch submit

Submits a batch of signed transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostSubmitBatchRequest**](ApiPostSubmitBatchRequest.md) |  | [required] |

### Return type

[**models::ApiPostSubmitBatchResponse**](apiPostSubmitBatchResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_submit_batch_v2_0

> models::ApiPostSubmitBatchResponse api_post_submit_batch_v2_0(body)
Transaction batch submit

Submits a batch of signed transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostSubmitBatchRequest**](ApiPostSubmitBatchRequest.md) |  | [required] |

### Return type

[**models::ApiPostSubmitBatchResponse**](apiPostSubmitBatchResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_submit_v2_0

> models::ApiPostSubmitResponse api_post_submit_v2_0(body)
Transaction submit

Submits a signed transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostSubmitRequest**](ApiPostSubmitRequest.md) |  | [required] |

### Return type

[**models::ApiPostSubmitResponse**](apiPostSubmitResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

