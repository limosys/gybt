# ApiPostOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_address** | **String** |  | 
**payer_address** | **String** |  | 
**market** | **String** |  | 
**side** | Option<[**models::ApiSide**](apiSide.md)> |  | [optional]
**r#type** | [**Vec<models::CommonOrderType>**](commonOrderType.md) |  | 
**amount** | **f64** |  | 
**price** | **f64** |  | 
**open_orders_address** | Option<**String**> | If left empty the Openbook API will lookup the account which can be time consuming | [optional]
**client_order_id** | Option<**String**> | Client defined OrderID | [optional]
**compute_limit** | Option<**i64**> | Optional: specifies total compute limit to be allocated for all instructions within the created tx | [optional]
**compute_price** | Option<**String**> | Optional: specifies compute price in microlamports to be included as a part of instruction, known as priority fee | [optional]
**tip** | Option<**String**> | Optional: specifies a tip amount that can be sent to bundle multiple transactions. Minimum value is 1025. | [optional]
**project** | Option<[**models::ApiProject**](apiProject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


