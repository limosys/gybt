# \RaydiumV2Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_raydium_clmm_pools**](RaydiumV2Api.md#api_get_raydium_clmm_pools) | **GET** /api/v2/raydium/clmm-pools | Raydium CLMM Pools
[**api_get_raydium_clmm_quotes**](RaydiumV2Api.md#api_get_raydium_clmm_quotes) | **GET** /api/v2/raydium/clmm-quotes | Raydium CLMM Quotes
[**api_get_raydium_cpmm_quotes**](RaydiumV2Api.md#api_get_raydium_cpmm_quotes) | **GET** /api/v2/raydium/cpmm-quotes | Raydium CLMM Quotes
[**api_get_raydium_pool_reserve**](RaydiumV2Api.md#api_get_raydium_pool_reserve) | **GET** /api/v2/raydium/pool-reserves | Raydium Pool reserve info
[**api_get_raydium_pools**](RaydiumV2Api.md#api_get_raydium_pools) | **GET** /api/v2/raydium/pools | Raydium Pools
[**api_get_raydium_prices**](RaydiumV2Api.md#api_get_raydium_prices) | **GET** /api/v2/raydium/prices | Token prices on Rayidum
[**api_get_raydium_quotes**](RaydiumV2Api.md#api_get_raydium_quotes) | **GET** /api/v2/raydium/quotes | Raydium Quotes
[**api_post_raydium_clmm_route_swap**](RaydiumV2Api.md#api_post_raydium_clmm_route_swap) | **POST** /api/v2/raydium/clmm-route-swap | Unsigned Raydium swap transaction along route
[**api_post_raydium_clmm_swap**](RaydiumV2Api.md#api_post_raydium_clmm_swap) | **POST** /api/v2/raydium/clmm-swap | Unsigned Raydium CLMM swap transaction
[**api_post_raydium_cpmm_swap**](RaydiumV2Api.md#api_post_raydium_cpmm_swap) | **POST** /api/v2/raydium/cpmm-swap | Unsigned Raydium CPMM swap transaction
[**api_post_raydium_route_swap**](RaydiumV2Api.md#api_post_raydium_route_swap) | **POST** /api/v2/raydium/route-swap | Unsigned Raydium swap transaction along route
[**api_post_raydium_swap**](RaydiumV2Api.md#api_post_raydium_swap) | **POST** /api/v2/raydium/swap | Unsigned Raydium swap transaction
[**api_post_raydium_swap_instructions**](RaydiumV2Api.md#api_post_raydium_swap_instructions) | **POST** /api/v2/raydium/swap-instructions | Unsigned Raydium swap transaction from instructions



## api_get_raydium_clmm_pools

> models::ApiGetRaydiumClmmPoolsResponse api_get_raydium_clmm_pools(pair_or_address)
Raydium CLMM Pools

Returns the list of Raydium CLMM pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pair_or_address** | Option<**String**> |  |  |

### Return type

[**models::ApiGetRaydiumClmmPoolsResponse**](apiGetRaydiumCLMMPoolsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_raydium_clmm_quotes

> models::ApiGetRaydiumClmmQuotesResponse api_get_raydium_clmm_quotes(in_token, out_token, in_amount, slippage)
Raydium CLMM Quotes

Returns quotes from Raydium CLMM pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_token** | Option<**String**> |  |  |
**out_token** | Option<**String**> |  |  |
**in_amount** | Option<**f64**> |  |  |
**slippage** | Option<**f64**> |  |  |

### Return type

[**models::ApiGetRaydiumClmmQuotesResponse**](apiGetRaydiumCLMMQuotesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_raydium_cpmm_quotes

> models::ApiGetRaydiumCpmmQuotesResponse api_get_raydium_cpmm_quotes(in_token, out_token, in_amount, slippage)
Raydium CLMM Quotes

Returns quotes from Raydium CPMM pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_token** | Option<**String**> |  |  |
**out_token** | Option<**String**> |  |  |
**in_amount** | Option<**f64**> |  |  |
**slippage** | Option<**f64**> |  |  |

### Return type

[**models::ApiGetRaydiumCpmmQuotesResponse**](apiGetRaydiumCPMMQuotesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_raydium_pool_reserve

> models::ApiGetRaydiumPoolReserveResponse api_get_raydium_pool_reserve(pairs_or_addresses)
Raydium Pool reserve info

Returns the Raydium pool reserve info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pairs_or_addresses** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::ApiGetRaydiumPoolReserveResponse**](apiGetRaydiumPoolReserveResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_raydium_pools

> models::ApiGetRaydiumPoolsResponse api_get_raydium_pools()
Raydium Pools

Returns the list of Raydium pools

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiGetRaydiumPoolsResponse**](apiGetRaydiumPoolsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_raydium_prices

> models::ApiGetRaydiumPricesResponse api_get_raydium_prices(tokens)
Token prices on Rayidum

Returns the list of prices for specified tokens on Raydium

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tokens** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::ApiGetRaydiumPricesResponse**](apiGetRaydiumPricesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_raydium_quotes

> models::ApiGetRaydiumQuotesResponse api_get_raydium_quotes(in_token, out_token, in_amount, slippage)
Raydium Quotes

Returns quotes from Raydium

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_token** | Option<**String**> |  |  |
**out_token** | Option<**String**> |  |  |
**in_amount** | Option<**f64**> |  |  |
**slippage** | Option<**f64**> |  |  |

### Return type

[**models::ApiGetRaydiumQuotesResponse**](apiGetRaydiumQuotesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_raydium_clmm_route_swap

> models::ApiPostRaydiumRouteSwapResponse api_post_raydium_clmm_route_swap(body)
Unsigned Raydium swap transaction along route

Generates an unsigned transaction object to make a CLMM swap on Raydium on the specified route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostRaydiumRouteSwapRequest**](ApiPostRaydiumRouteSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostRaydiumRouteSwapResponse**](apiPostRaydiumRouteSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_raydium_clmm_swap

> models::ApiPostRaydiumSwapResponse api_post_raydium_clmm_swap(body)
Unsigned Raydium CLMM swap transaction

Generates an unsigned transaction object to make a swap on Raydium CLMMs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostRaydiumSwapRequest**](ApiPostRaydiumSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostRaydiumSwapResponse**](apiPostRaydiumSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_raydium_cpmm_swap

> models::ApiPostRaydiumCpmmSwapResponse api_post_raydium_cpmm_swap(body)
Unsigned Raydium CPMM swap transaction

Generates an unsigned transaction object to make a swap on Raydium CPMMs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostRaydiumCpmmSwapRequest**](ApiPostRaydiumCpmmSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostRaydiumCpmmSwapResponse**](apiPostRaydiumCPMMSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_raydium_route_swap

> models::ApiPostRaydiumRouteSwapResponse api_post_raydium_route_swap(body)
Unsigned Raydium swap transaction along route

Generates an unsigned transaction object to make a swap on Raydium on the specified route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostRaydiumRouteSwapRequest**](ApiPostRaydiumRouteSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostRaydiumRouteSwapResponse**](apiPostRaydiumRouteSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_raydium_swap

> models::ApiPostRaydiumSwapResponse api_post_raydium_swap(body)
Unsigned Raydium swap transaction

Generates an unsigned transaction object to make a swap on Raydium

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostRaydiumSwapRequest**](ApiPostRaydiumSwapRequest.md) |  | [required] |

### Return type

[**models::ApiPostRaydiumSwapResponse**](apiPostRaydiumSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_raydium_swap_instructions

> models::ApiPostRaydiumSwapInstructionsResponse api_post_raydium_swap_instructions(body)
Unsigned Raydium swap transaction from instructions

Generates an unsigned transaction object using raydium instructions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostRaydiumSwapInstructionsRequest**](ApiPostRaydiumSwapInstructionsRequest.md) |  | [required] |

### Return type

[**models::ApiPostRaydiumSwapInstructionsResponse**](apiPostRaydiumSwapInstructionsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

