# V1ResourceQuotaSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hard** | Option<**::std::collections::HashMap<String, String>**> | hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/ | [optional]
**scope_selector** | Option<[**crate::models::V1ScopeSelector**](v1.ScopeSelector.md)> |  | [optional]
**scopes** | Option<**Vec<String>**> | A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


