# \OreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_post_submit_mine_ore**](OreApi.md#api_post_submit_mine_ore) | **POST** /api/v2/mine-ore | Submit mine ore transactions



## api_post_submit_mine_ore

> models::ApiPostSubmitResponse api_post_submit_mine_ore(body)
Submit mine ore transactions

Submits two transactions in sequence for mining ore

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

