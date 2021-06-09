# V1ReplicaSetSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | Option<**i32**> | Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) | [optional]
**replicas** | Option<**i32**> | Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller | [optional]
**selector** | [**crate::models::V1LabelSelector**](v1.LabelSelector.md) |  | 
**template** | Option<[**crate::models::V1PodTemplateSpec**](v1.PodTemplateSpec.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


