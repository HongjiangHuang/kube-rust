# V1CustomResourceDefinitionStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_names** | Option<[**crate::models::V1CustomResourceDefinitionNames**](v1.CustomResourceDefinitionNames.md)> |  | [optional]
**conditions** | Option<[**Vec<crate::models::V1CustomResourceDefinitionCondition>**](v1.CustomResourceDefinitionCondition.md)> | conditions indicate state for particular aspects of a CustomResourceDefinition | [optional]
**stored_versions** | Option<**Vec<String>**> | storedVersions lists all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so a migration controller can finish a migration to another version (ensuring no old objects are left in storage), and then remove the rest of the versions from this list. Versions may not be removed from `spec.versions` while they exist in this list. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


