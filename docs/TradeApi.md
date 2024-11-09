# \TradeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_open_orders**](TradeApi.md#api_get_open_orders) | **GET** /api/v1/trade/openorders/{market} | List of user's open orders
[**api_get_unsettled**](TradeApi.md#api_get_unsettled) | **GET** /api/v1/trade/unsettled/{market} | Unsettled amounts
[**api_post_cancel_all**](TradeApi.md#api_post_cancel_all) | **POST** /api/v1/trade/cancelall | Unsigned CancelAll transaction
[**api_post_cancel_by_client_order_id**](TradeApi.md#api_post_cancel_by_client_order_id) | **POST** /api/v1/trade/cancelbyid | Unsigned CancelOrderByID transaction
[**api_post_cancel_order**](TradeApi.md#api_post_cancel_order) | **POST** /api/v1/trade/cancel | Unsigned CancelOrder transaction
[**api_post_order**](TradeApi.md#api_post_order) | **POST** /api/v1/trade/place | Unsigned NewOrderV3 transaction
[**api_post_replace_by_client_order_id**](TradeApi.md#api_post_replace_by_client_order_id) | **POST** /api/v1/trade/replacebyclientid | Unsigned ReplaceByClientOrderID transaction
[**api_post_replace_order**](TradeApi.md#api_post_replace_order) | **POST** /api/v1/trade/replace | Unsigned ReplaceOrder transaction
[**api_post_route_trade_swap**](TradeApi.md#api_post_route_trade_swap) | **POST** /api/v1/trade/route-swap | Unsigned AMM swap transaction along route
[**api_post_settle**](TradeApi.md#api_post_settle) | **POST** /api/v1/trade/settle | Unsigned SettleFunds transaction
[**api_post_submit**](TradeApi.md#api_post_submit) | **POST** /api/v1/trade/submit | Transaction submit
[**api_post_submit_batch**](TradeApi.md#api_post_submit_batch) | **POST** /api/v1/trade/submit-batch | Transaction batch submit
[**api_post_submit_batch_v2**](TradeApi.md#api_post_submit_batch_v2) | **POST** /api/v2/submit-batch | Transaction batch submit
[**api_post_submit_v2**](TradeApi.md#api_post_submit_v2) | **POST** /api/v2/submit | Transaction submit
[**api_post_trade_swap**](TradeApi.md#api_post_trade_swap) | **POST** /api/v1/trade/swap | Unsigned best available AMM swap transaction



## api_get_open_orders

> models::ApiGetOpenOrdersResponse api_get_open_orders(market, limit, address, open_orders_address, project)
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


## api_get_unsettled

> models::ApiGetUnsettledResponse api_get_unsettled(market, owner_address, project)
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


## api_post_cancel_all

> models::ApiPostCancelAllResponse api_post_cancel_all(body)
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


## api_post_cancel_by_client_order_id

> models::ApiPostCancelOrderResponse api_post_cancel_by_client_order_id(body)
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


## api_post_cancel_order

> models::ApiPostCancelOrderResponse api_post_cancel_order(body)
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


## api_post_order

> models::ApiPostOrderResponse api_post_order(body)
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


## api_post_replace_by_client_order_id

> models::ApiPostOrderResponse api_post_replace_by_client_order_id(body)
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


## api_post_replace_order

> models::ApiPostOrderResponse api_post_replace_order(body)
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


## api_post_route_trade_swap

> models::ApiTradeSwapResponse api_post_route_trade_swap(body)
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


## api_post_settle

> models::ApiPostSettleResponse api_post_settle(body)
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


## api_post_submit

> models::ApiPostSubmitResponse api_post_submit(body)
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


## api_post_submit_batch

> models::ApiPostSubmitBatchResponse api_post_submit_batch(body)
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


## api_post_submit_batch_v2

> models::ApiPostSubmitBatchResponse api_post_submit_batch_v2(body)
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


## api_post_submit_v2

> models::ApiPostSubmitResponse api_post_submit_v2(body)
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


## api_post_trade_swap

> models::ApiTradeSwapResponse api_post_trade_swap(body)
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

