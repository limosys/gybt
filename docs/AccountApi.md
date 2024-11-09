# \AccountApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_get_account_balance**](AccountApi.md#api_get_account_balance) | **GET** /api/v1/account/balance | Token balances
[**api_get_account_balance_v2**](AccountApi.md#api_get_account_balance_v2) | **GET** /api/v2/balance | Token balances
[**api_get_token_accounts**](AccountApi.md#api_get_token_accounts) | **GET** /api/v1/account/token-accounts | Token accounts with balances



## api_get_account_balance

> models::ApiGetAccountBalanceResponse api_get_account_balance(owner_address)
Token balances

Returns the all token balances for a wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_address** | **String** |  | [required] |

### Return type

[**models::ApiGetAccountBalanceResponse**](apiGetAccountBalanceResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_account_balance_v2

> models::ApiGetAccountBalanceResponse api_get_account_balance_v2(owner_address)
Token balances

Returns the all token balances for a wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_address** | **String** |  | [required] |

### Return type

[**models::ApiGetAccountBalanceResponse**](apiGetAccountBalanceResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_get_token_accounts

> models::ApiGetTokenAccountsResponse api_get_token_accounts(owner_address)
Token accounts with balances

Returns the all token accounts with their balances for a wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_address** | **String** |  | [required] |

### Return type

[**models::ApiGetTokenAccountsResponse**](apiGetTokenAccountsResponse.md)

### Authorization

[Auth Header](../README.md#Auth Header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

