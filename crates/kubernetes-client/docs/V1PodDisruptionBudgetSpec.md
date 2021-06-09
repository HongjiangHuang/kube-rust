# V1PodDisruptionBudgetSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_unavailable** | Option<[**serde_json::Value**](.md)> | An eviction is allowed if at most \"maxUnavailable\" pods selected by \"selector\" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with \"minAvailable\". | [optional]
**min_available** | Option<[**serde_json::Value**](.md)> | An eviction is allowed if at least \"minAvailable\" pods selected by \"selector\" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying \"100%\". | [optional]
**selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


