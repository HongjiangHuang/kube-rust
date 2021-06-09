# V1ManagedFieldsEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | Option<**String**> | APIVersion defines the version of this resource that this field set applies to. The format is \"group/version\" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted. | [optional]
**fields_type** | Option<**String**> | FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: \"FieldsV1\" | [optional]
**fields_v1** | Option<[**serde_json::Value**](.md)> | FieldsV1 holds the first JSON version format as described in the \"FieldsV1\" type. | [optional]
**manager** | Option<**String**> | Manager is an identifier of the workflow managing these fields. | [optional]
**operation** | Option<**String**> | Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'. | [optional]
**time** | Option<**String**> | Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


