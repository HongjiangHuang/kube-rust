# V1alpha1RuntimeClassSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**overhead** | Option<[**crate::models::V1alpha1Overhead**](v1alpha1.Overhead.md)> |  | [optional]
**runtime_handler** | **String** | RuntimeHandler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called \"runc\" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The RuntimeHandler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable. | 
**scheduling** | Option<[**crate::models::V1alpha1Scheduling**](v1alpha1.Scheduling.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


