# \OrderbookApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_markets_0**](OrderbookApi.md#api_get_markets_0) | **GET** /api/v1/market/markets | Orderbook markets
[**api_get_open_orders_0**](OrderbookApi.md#api_get_open_orders_0) | **GET** /api/v1/trade/openorders/{market} | List of user's open orders
[**api_get_orderbook_0**](OrderbookApi.md#api_get_orderbook_0) | **GET** /api/v1/market/orderbooks/{market} | Market orderbook
[**api_get_tickers_0**](OrderbookApi.md#api_get_tickers_0) | **GET** /api/v1/market/tickers/{market} | Orderbook tickers
[**api_get_unsettled_0**](OrderbookApi.md#api_get_unsettled_0) | **GET** /api/v1/trade/unsettled/{market} | Unsettled amounts
[**api_post_cancel_all_0**](OrderbookApi.md#api_post_cancel_all_0) | **POST** /api/v1/trade/cancelall | Unsigned CancelAll transaction
[**api_post_cancel_by_client_order_id_0**](OrderbookApi.md#api_post_cancel_by_client_order_id_0) | **POST** /api/v1/trade/cancelbyid | Unsigned CancelOrderByID transaction
[**api_post_cancel_order_0**](OrderbookApi.md#api_post_cancel_order_0) | **POST** /api/v1/trade/cancel | Unsigned CancelOrder transaction
[**api_post_order_0**](OrderbookApi.md#api_post_order_0) | **POST** /api/v1/trade/place | Unsigned NewOrderV3 transaction
[**api_post_replace_by_client_order_id_0**](OrderbookApi.md#api_post_replace_by_client_order_id_0) | **POST** /api/v1/trade/replacebyclientid | Unsigned ReplaceByClientOrderID transaction
[**api_post_replace_order_0**](OrderbookApi.md#api_post_replace_order_0) | **POST** /api/v1/trade/replace | Unsigned ReplaceOrder transaction
[**api_post_settle_0**](OrderbookApi.md#api_post_settle_0) | **POST** /api/v1/trade/settle | Unsigned SettleFunds transaction



## api_get_markets_0

> models::ApiGetMarketsResponse api_get_markets_0()
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


## api_get_open_orders_0

> models::ApiGetOpenOrdersResponse api_get_open_orders_0(market, limit, address, open_orders_address, project)
List of user's open orders

Returns the list of open orders per user per market

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**limit** | Option<**i64**> | Use 0 for no limits |  |
**address** | Option<**String**> |  |  |
**open_orders_address** | Option<**String**> | If left empty the Serum API will lookup the account which can be time consuming, Setting both address and openOrdersAddress is invalid, openOrdersAddress is preferred. |  |
**project** | Option<**String**> |  |  |[default to P_UNKNOWN]

### Return type

[**models::ApiGetOpenOrdersResponse**](apiGetOpenOrdersResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_orderbook_0

> models::ApiGetOrderbookResponse api_get_orderbook_0(market, limit, project)
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


## api_get_tickers_0

> models::ApiGetTickersResponse api_get_tickers_0(market, project)
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


## api_get_unsettled_0

> models::ApiGetUnsettledResponse api_get_unsettled_0(market, owner_address, project)
Unsettled amounts

Returns the unsettled amounts of user in a market

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** |  | [required] |
**owner_address** | **String** |  | [required] |
**project** | Option<**String**> |  |  |[default to P_UNKNOWN]

### Return type

[**models::ApiGetUnsettledResponse**](apiGetUnsettledResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_cancel_all_0

> models::ApiPostCancelAllResponse api_post_cancel_all_0(body)
Unsigned CancelAll transaction

Generates a CancelAll unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostCancelAllRequest**](ApiPostCancelAllRequest.md) |  | [required] |

### Return type

[**models::ApiPostCancelAllResponse**](apiPostCancelAllResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_cancel_by_client_order_id_0

> models::ApiPostCancelOrderResponse api_post_cancel_by_client_order_id_0(body)
Unsigned CancelOrderByID transaction

Generates a CancelOrderByID unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostCancelByClientOrderIdRequest**](ApiPostCancelByClientOrderIdRequest.md) |  | [required] |

### Return type

[**models::ApiPostCancelOrderResponse**](apiPostCancelOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_cancel_order_0

> models::ApiPostCancelOrderResponse api_post_cancel_order_0(body)
Unsigned CancelOrder transaction

Generates a CancelOrder unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostCancelOrderRequest**](ApiPostCancelOrderRequest.md) |  | [required] |

### Return type

[**models::ApiPostCancelOrderResponse**](apiPostCancelOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_order_0

> models::ApiPostOrderResponse api_post_order_0(body)
Unsigned NewOrderV3 transaction

Generates a NewOrderV3 unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostOrderRequest**](ApiPostOrderRequest.md) |  | [required] |

### Return type

[**models::ApiPostOrderResponse**](apiPostOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_replace_by_client_order_id_0

> models::ApiPostOrderResponse api_post_replace_by_client_order_id_0(body)
Unsigned ReplaceByClientOrderID transaction

Generates a ReplaceByClientOrderID unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostOrderRequest**](ApiPostOrderRequest.md) |  | [required] |

### Return type

[**models::ApiPostOrderResponse**](apiPostOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_replace_order_0

> models::ApiPostOrderResponse api_post_replace_order_0(body)
Unsigned ReplaceOrder transaction

Generates a ReplaceOrder unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostReplaceOrderRequest**](ApiPostReplaceOrderRequest.md) |  | [required] |

### Return type

[**models::ApiPostOrderResponse**](apiPostOrderResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_post_settle_0

> models::ApiPostSettleResponse api_post_settle_0(body)
Unsigned SettleFunds transaction

Generates a SettleFunds unsigned transaction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiPostSettleRequest**](ApiPostSettleRequest.md) |  | [required] |

### Return type

[**models::ApiPostSettleResponse**](apiPostSettleResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

