# V1IngressClassParametersReference

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_group** | Option<**String**> | APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required. | [optional]
**kind** | **String** | Kind is the type of resource being referenced. | 
**name** | **String** | Name is the name of resource being referenced. | 
**namespace** | Option<**String**> | Namespace is the namespace of the resource being referenced. This field is required when scope is set to \"Namespace\" and must be unset when scope is set to \"Cluster\". | [optional]
**scope** | Option<**String**> | Scope represents if this refers to a cluster or namespace scoped resource. This may be set to \"Cluster\" (default) or \"Namespace\". Field can be enabled with IngressClassNamespacedParams feature gate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


