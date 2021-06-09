# V1ContainerStateTerminated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_id** | Option<**String**> | Container's ID in the format 'docker://<container_id>' | [optional]
**exit_code** | **i32** | Exit status from the last termination of the container | 
**finished_at** | Option<**String**> | Time at which the container last terminated | [optional]
**message** | Option<**String**> | Message regarding the last termination of the container | [optional]
**reason** | Option<**String**> | (brief) reason from the last termination of the container | [optional]
**signal** | Option<**i32**> | Signal from the last termination of the container | [optional]
**started_at** | Option<**String**> | Time at which previous execution of the container started | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


