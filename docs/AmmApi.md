# \AmmApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_pools_0**](AmmApi.md#api_get_pools_0) | **GET** /api/v1/market/pools | AMM Pools
[**api_get_quotes_0**](AmmApi.md#api_get_quotes_0) | **GET** /api/v1/market/quote | AMM Quotes
[**api_post_route_trade_swap_0**](AmmApi.md#api_post_route_trade_swap_0) | **POST** /api/v1/trade/route-swap | Unsigned AMM swap transaction along route
[**api_post_trade_swap_0**](AmmApi.md#api_post_trade_swap_0) | **POST** /api/v1/trade/swap | Unsigned best available AMM swap transaction



## api_get_pools_0

> models::ApiGetPoolsResponse api_get_pools_0(projects, pair_or_address)
AMM Pools

Returns the list of supported AMM pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects** | Option<[**Vec<String>**](String.md)> |  |  |
**pair_or_address** | Option<**String**> |  |  |

### Return type

[**models::ApiGetPoolsResponse**](apiGetPoolsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_quotes_0

> models::ApiGetQuotesResponse api_get_quotes_0(in_token, out_token, in_amount, slippage, limit, projects)
AMM Quotes

Returns quotes from supported AMMs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**in_token** | Option<**String**> |  |  |
**out_token** | Option<**String**> |  |  |
**in_amount** | Option<**f64**> |  |  |
**slippage** | Option<**f64**> |  |  |
**limit** | Option<**i32**> |  |  |
**projects** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::ApiGetQuotesResponse**](apiGetQuotesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_route_trade_swap_0

> models::ApiTradeSwapResponse api_post_route_trade_swap_0(body)
Unsigned AMM swap transaction along route

Generates an unsigned transaction object for the AMM swap on the specified route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiRouteTradeSwapRequest**](ApiRouteTradeSwapRequest.md) |  | [required] |

### Return type

[**models::ApiTradeSwapResponse**](apiTradeSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_trade_swap_0

> models::ApiTradeSwapResponse api_post_trade_swap_0(body)
Unsigned best available AMM swap transaction

Generates an unsigned transaction object for the best available AMM swap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiTradeSwapRequest**](ApiTradeSwapRequest.md) |  | [required] |

### Return type

[**models::ApiTradeSwapResponse**](apiTradeSwapResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

