# V1CsiPersistentVolumeSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**controller_expand_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**controller_publish_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**driver** | **String** | Driver is the name of the driver to use for this volume. Required. | 
**fs_type** | Option<**String**> | Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". | [optional]
**node_publish_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**node_stage_secret_ref** | Option<[**crate::models::V1SecretReference**](v1.SecretReference.md)> |  | [optional]
**read_only** | Option<**bool**> | Optional: The value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write). | [optional]
**volume_attributes** | Option<**::std::collections::HashMap<String, String>**> | Attributes of the volume to publish. | [optional]
**volume_handle** | **String** | VolumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


