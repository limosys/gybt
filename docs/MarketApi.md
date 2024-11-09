# \MarketApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_market_depth**](MarketApi.md#api_get_market_depth) | **GET** /api/v1/market/depth/{market} | Market depth
[**api_get_markets**](MarketApi.md#api_get_markets) | **GET** /api/v1/market/markets | Orderbook markets
[**api_get_orderbook**](MarketApi.md#api_get_orderbook) | **GET** /api/v1/market/orderbooks/{market} | Market orderbook
[**api_get_pools**](MarketApi.md#api_get_pools) | **GET** /api/v1/market/pools | AMM Pools
[**api_get_price**](MarketApi.md#api_get_price) | **GET** /api/v1/market/price | Token prices
[**api_get_quotes**](MarketApi.md#api_get_quotes) | **GET** /api/v1/market/quote | AMM Quotes
[**api_get_tickers**](MarketApi.md#api_get_tickers) | **GET** /api/v1/market/tickers/{market} | Orderbook tickers



## api_get_market_depth

> models::ApiGetMarketDepthResponse api_get_market_depth(market, limit, project)
Market depth

Returns market's aggregated price data. Use limit param to reduce the number of price points returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**limit** | Option<**i64**> | Use 0 for no limits |  |
**project** | Option<**String**> |  |  |[default to P_UNKNOWN]

### Return type

[**models::ApiGetMarketDepthResponse**](apiGetMarketDepthResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_markets

> models::ApiGetMarketsResponse api_get_markets()
Orderbook markets

Returns the list of orderbook markets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiGetMarketsResponse**](apiGetMarketsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_orderbook

> models::ApiGetOrderbookResponse api_get_orderbook(market, limit, project)
Market orderbook

Returns market's orderbook. Use limit param to reduce the number of bids/asks returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**limit** | Option<**i64**> | Use 0 for no limits |  |
**project** | Option<**String**> |  |  |[default to P_UNKNOWN]

### Return type

[**models::ApiGetOrderbookResponse**](apiGetOrderbookResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_pools

> models::ApiGetPoolsResponse api_get_pools(projects, pair_or_address)
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


## api_get_price

> models::ApiGetPriceResponse api_get_price(tokens)
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


## api_get_quotes

> models::ApiGetQuotesResponse api_get_quotes(in_token, out_token, in_amount, slippage, limit, projects)
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


## api_get_tickers

> models::ApiGetTickersResponse api_get_tickers(market, project)
Orderbook tickers

Returns updated ticker(s). To receive all tickers use empty string for {market} param value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** | Use empty string for all markets | [required] |
**project** | Option<**String**> |  |  |[default to P_UNKNOWN]

### Return type

[**models::ApiGetTickersResponse**](apiGetTickersResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

