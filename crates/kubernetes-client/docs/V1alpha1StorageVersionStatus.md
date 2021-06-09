# V1alpha1StorageVersionStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**common_encoding_version** | Option<**String**> | If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality. | [optional]
**conditions** | Option<[**Vec<crate::models::V1alpha1StorageVersionCondition>**](v1alpha1.StorageVersionCondition.md)> | The latest available observations of the storageVersion's state. | [optional]
**storage_versions** | Option<[**Vec<crate::models::V1alpha1ServerStorageVersion>**](v1alpha1.ServerStorageVersion.md)> | The reported versions per API server instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


