# ApiPostSettleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_address** | **String** |  | 
**market** | **String** |  | 
**base_token_wallet** | **String** |  | 
**quote_token_wallet** | **String** |  | 
**open_orders_address** | Option<**String**> | If left empty the Serum API will lookup the account which can be time consuming | [optional]
**compute_limit** | Option<**i64**> | Optional: specifies total compute limit to be allocated for all instructions within the created tx | [optional]
**compute_price** | Option<**String**> | Optional: specifies compute price in microlamports to be included as a part of instruction, known as priority fee | [optional]
**project** | Option<[**models::ApiProject**](apiProject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


