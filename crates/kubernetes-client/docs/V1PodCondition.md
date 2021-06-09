# V1PodCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | Option<**String**> | Last time we probed the condition. | [optional]
**last_transition_time** | Option<**String**> | Last time the condition transitioned from one status to another. | [optional]
**message** | Option<**String**> | Human-readable message indicating details about last transition. | [optional]
**reason** | Option<**String**> | Unique, one-word, CamelCase reason for the condition's last transition. | [optional]
**status** | **String** | Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions | 
**_type** | **String** | Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


