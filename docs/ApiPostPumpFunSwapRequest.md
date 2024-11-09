# ApiPostPumpFunSwapRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_address** | Option<**String**> |  | [optional]
**bonding_curve_address** | Option<**String**> |  | [optional]
**token_address** | Option<**String**> |  | [optional]
**token_amount** | Option<**f64**> |  | [optional]
**sol_threshold** | Option<**f64**> |  | [optional]
**is_buy** | Option<**bool**> |  | [optional]
**compute_limit** | Option<**i64**> | Optional: specifies total compute limit to be allocated for all instructions within the created tx | [optional]
**compute_price** | Option<**String**> | Optional: specifies compute price in microlamports to be included as a part of instruction, known as priority fee | [optional]
**tip** | Option<**String**> | Optional: Specifies a tip amount that will be used to pay for front-running protection and/or bundle submission services. Minimum value is 1025 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


