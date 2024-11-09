# ApiPostJupiterSwapRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_address** | Option<**String**> |  | [optional]
**in_token** | Option<**String**> |  | [optional]
**out_token** | Option<**String**> |  | [optional]
**in_amount** | Option<**f64**> |  | [optional]
**slippage** | Option<**f64**> |  | [optional]
**compute_limit** | Option<**i64**> | Optional: specifies total compute limit to be allocated for all instructions within the created tx | [optional]
**compute_price** | Option<**String**> | Optional: specifies compute price in microlamports to be included as a part of instruction, known as priority fee | [optional]
**tip** | Option<**String**> | Optional: Specifies a tip amount that will be used to pay for front-running protection and/or bundle submission services. Minimum value is 1025 | [optional]
**fast_mode** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


