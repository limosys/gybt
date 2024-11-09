# \JupiterV2Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_jupiter_prices**](JupiterV2Api.md#api_get_jupiter_prices) | **GET** /api/v2/jupiter/prices | Token prices on Jupiter
[**api_get_jupiter_quotes**](JupiterV2Api.md#api_get_jupiter_quotes) | **GET** /api/v2/jupiter/quotes | Jupiter Quotes
[**api_post_jupiter_route_swap**](JupiterV2Api.md#api_post_jupiter_route_swap) | **POST** /api/v2/jupiter/route-swap | Unsigned Jupiter swap transaction along route
[**api_post_jupiter_swap**](JupiterV2Api.md#api_post_jupiter_swap) | **POST** /api/v2/jupiter/swap | Unsigned Jupiter swap transaction
[**api_post_jupiter_swap_instructions**](JupiterV2Api.md#api_post_jupiter_swap_instructions) | **POST** /api/v2/jupiter/swap-instructions | Unsigned Jupiter swap transaction from instructions



## api_get_jupiter_prices

> models::ApiGetJupiterPricesResponse api_get_jupiter_prices(tokens)
Token prices on Jupiter

Returns the list of prices for specified tokens on Jupiter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tokens** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::ApiGetJupiterPricesResponse**](apiGetJupiterPricesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_jupiter_quotes

> models::ApiGetJupiterQuotesResponse api_get_jupiter_quotes(in_token, out_token, in_amount, slippage, fast_mode)
Jupiter Quotes

Returns quotes from Jupiter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_token** | Option<**String**> |  |  |
**out_token** | Option<**String**> |  |  |
**in_amount** | Option<**f64**> |  |  |
**slippage** | Option<**f64**> |  |  |
**fast_mode** | Option<**bool**> |  |  |

### Return type

[**models::ApiGetJupiterQuotesResponse**](apiGetJupiterQuotesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_jupiter_route_swap

> models::ApiPostJupiterRouteSwapResponse api_post_jupiter_route_swap(body)
Unsigned Jupiter swap transaction along route

Generates an unsigned transaction object to make a swap on Jupiter on the specified route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostJupiterRouteSwapRequest**](ApiPostJupiterRouteSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostJupiterRouteSwapResponse**](apiPostJupiterRouteSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_jupiter_swap

> models::ApiPostJupiterSwapResponse api_post_jupiter_swap(body)
Unsigned Jupiter swap transaction

Generates an unsigned transaction object to make a swap on Jupiter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostJupiterSwapRequest**](ApiPostJupiterSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostJupiterSwapResponse**](apiPostJupiterSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_jupiter_swap_instructions

> models::ApiPostJupiterSwapInstructionsResponse api_post_jupiter_swap_instructions(body)
Unsigned Jupiter swap transaction from instructions

Generates an unsigned transaction object using jupiter instructions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostJupiterSwapInstructionsRequest**](ApiPostJupiterSwapInstructionsRequest.md) |  | [required] |

### Return type

[**models::ApiPostJupiterSwapInstructionsResponse**](apiPostJupiterSwapInstructionsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

