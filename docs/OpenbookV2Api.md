# \OpenbookV2Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_market_depth_v2**](OpenbookV2Api.md#api_get_market_depth_v2) | **GET** /api/v2/openbook/depth/{market} | Openbook market depth
[**api_get_markets_v2**](OpenbookV2Api.md#api_get_markets_v2) | **GET** /api/v2/openbook/markets | Openbook markets
[**api_get_open_orders_v2**](OpenbookV2Api.md#api_get_open_orders_v2) | **GET** /api/v2/openbook/open-orders/{market} | List of user's open orders in Openbook
[**api_get_orderbook_v2**](OpenbookV2Api.md#api_get_orderbook_v2) | **GET** /api/v2/openbook/orderbooks/{market} | Openbook market orderbook
[**api_get_tickers_v2**](OpenbookV2Api.md#api_get_tickers_v2) | **GET** /api/v2/openbook/tickers/{market} | Openbook tickers
[**api_get_unsettled_v2**](OpenbookV2Api.md#api_get_unsettled_v2) | **GET** /api/v2/openbook/unsettled/{market} | Unsettled amounts
[**api_post_cancel_order_v2**](OpenbookV2Api.md#api_post_cancel_order_v2) | **POST** /api/v2/openbook/cancel | Openbook unsigned CancelOrder transaction
[**api_post_order_v2**](OpenbookV2Api.md#api_post_order_v2) | **POST** /api/v2/openbook/place | Openbook unsigned NewOrderV3 transaction
[**api_post_replace_order_v2**](OpenbookV2Api.md#api_post_replace_order_v2) | **POST** /api/v2/openbook/replace | Openbook Unsigned ReplaceOrder transaction
[**api_post_settle_v2**](OpenbookV2Api.md#api_post_settle_v2) | **POST** /api/v2/openbook/settle | Openbook unsigned SettleFunds transaction



## api_get_market_depth_v2

> models::ApiGetMarketDepthResponseV2 api_get_market_depth_v2(market, limit)
Openbook market depth

Returns market's aggregated price data. Use limit param to reduce the number of price points returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**limit** | Option<**i64**> | Use 0 for no limits |  |

### Return type

[**models::ApiGetMarketDepthResponseV2**](apiGetMarketDepthResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_markets_v2

> models::ApiGetMarketsResponseV2 api_get_markets_v2()
Openbook markets

Returns the list of orderbook markets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiGetMarketsResponseV2**](apiGetMarketsResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_open_orders_v2

> models::ApiGetOpenOrdersResponseV2 api_get_open_orders_v2(market, limit, address, open_orders_address, order_id, client_order_id)
List of user's open orders in Openbook

Returns the list of open orders per user per market

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**limit** | Option<**i64**> | Use 0 for no limits |  |
**address** | Option<**String**> |  |  |
**open_orders_address** | Option<**String**> | If left empty the Serum API will lookup the account which can be time consuming, Setting both address and openOrdersAddress is invalid, openOrdersAddress is preferred. |  |
**order_id** | Option<**String**> |  |  |
**client_order_id** | Option<**String**> |  |  |

### Return type

[**models::ApiGetOpenOrdersResponseV2**](apiGetOpenOrdersResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_orderbook_v2

> models::ApiGetOrderbookResponseV2 api_get_orderbook_v2(market, limit)
Openbook market orderbook

Returns market's orderbook. Use limit param to reduce the number of bids/asks returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**limit** | Option<**i64**> | Use 0 for no limits |  |

### Return type

[**models::ApiGetOrderbookResponseV2**](apiGetOrderbookResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_tickers_v2

> models::ApiGetTickersResponseV2 api_get_tickers_v2(market)
Openbook tickers

Returns updated ticker(s). To receive all tickers use empty string for {market} param value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** | Use empty string for all markets | [required] |

### Return type

[**models::ApiGetTickersResponseV2**](apiGetTickersResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_unsettled_v2

> models::ApiGetUnsettledResponse api_get_unsettled_v2(market, owner_address)
Unsettled amounts

Returns the unsettled amounts of user in a market

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**owner_address** | **String** |  | [required] |

### Return type

[**models::ApiGetUnsettledResponse**](apiGetUnsettledResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_cancel_order_v2

> models::ApiPostCancelOrderResponseV2 api_post_cancel_order_v2(body)
Openbook unsigned CancelOrder transaction

Generates a CancelOrder unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostCancelOrderRequestV2**](ApiPostCancelOrderRequestV2.md) |  | [required] |

### Return type

[**models::ApiPostCancelOrderResponseV2**](apiPostCancelOrderResponseV2.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_order_v2

> models::ApiPostOrderResponse api_post_order_v2(body)
Openbook unsigned NewOrderV3 transaction

Generates a NewOrderV3 unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostOrderRequestV2**](ApiPostOrderRequestV2.md) |  | [required] |

### Return type

[**models::ApiPostOrderResponse**](apiPostOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_replace_order_v2

> models::ApiPostOrderResponse api_post_replace_order_v2(body)
Openbook Unsigned ReplaceOrder transaction

Generates a ReplaceOrder unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostReplaceOrderRequestV2**](ApiPostReplaceOrderRequestV2.md) |  | [required] |

### Return type

[**models::ApiPostOrderResponse**](apiPostOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_settle_v2

> models::ApiPostSettleResponse api_post_settle_v2(body)
Openbook unsigned SettleFunds transaction

Generates a SettleFunds unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostSettleRequestV2**](ApiPostSettleRequestV2.md) |  | [required] |

### Return type

[**models::ApiPostSettleResponse**](apiPostSettleResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

