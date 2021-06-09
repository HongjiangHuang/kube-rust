# V1PersistentVolumeClaimCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | Option<**String**> | Last time we probed the condition. | [optional]
**last_transition_time** | Option<**String**> | Last time the condition transitioned from one status to another. | [optional]
**message** | Option<**String**> | Human-readable message indicating details about last transition. | [optional]
**reason** | Option<**String**> | Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"ResizeStarted\" that means the underlying persistent volume is being resized. | [optional]
**status** | **String** |  | 
**_type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


