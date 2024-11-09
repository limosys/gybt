# \PumpFunApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_pump_fun_quotes**](PumpFunApi.md#api_get_pump_fun_quotes) | **GET** /api/v2/pumpfun/quotes | PumpFun Quotes



## api_get_pump_fun_quotes

> models::ApiGetPumpFunQuotesResponse api_get_pump_fun_quotes(quote_type, mint_address, bonding_curve_address, amount, slippage)
PumpFun Quotes

Returns quotes from PumpFun

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_type** | Option<**String**> |  |  |
**mint_address** | Option<**String**> |  |  |
**bonding_curve_address** | Option<**String**> |  |  |
**amount** | Option<**f64**> |  |  |
**slippage** | Option<**f64**> |  |  |

### Return type

[**models::ApiGetPumpFunQuotesResponse**](apiGetPumpFunQuotesResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

