# ApiPostJupiterSwapInstructionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instructions** | Option<[**Vec<models::ApiInstructionJupiter>**](apiInstructionJupiter.md)> |  | [optional]
**address_lookup_table_addresses** | Option<[**std::collections::HashMap<String, models::ApiPublicKeys>**](apiPublicKeys.md)> | Returns a map[publicKey][]publicKey that is used in solana versioned transaction creation. | [optional]
**out_amount** | Option<**f64**> |  | [optional]
**out_amount_min** | Option<**f64**> |  | [optional]
**price_impact** | Option<[**models::CommonPriceImpactPercentV2**](commonPriceImpactPercentV2.md)> |  | [optional]
**fees** | Option<[**Vec<models::CommonFee>**](commonFee.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


