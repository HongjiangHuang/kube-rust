# V1beta1LimitResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queuing** | Option<[**crate::models::V1beta1QueuingConfiguration**](v1beta1.QueuingConfiguration.md)> |  | [optional]
**_type** | **String** | `type` is \"Queue\" or \"Reject\". \"Queue\" means that requests that can not be executed upon arrival are held in a queue until they can be executed or a queuing limit is reached. \"Reject\" means that requests that can not be executed upon arrival are rejected. Required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


