# V1Volume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_elastic_block_store** | Option<[**crate::models::V1AwsElasticBlockStoreVolumeSource**](v1.AWSElasticBlockStoreVolumeSource.md)> |  | [optional]
**azure_disk** | Option<[**crate::models::V1AzureDiskVolumeSource**](v1.AzureDiskVolumeSource.md)> |  | [optional]
**azure_file** | Option<[**crate::models::V1AzureFileVolumeSource**](v1.AzureFileVolumeSource.md)> |  | [optional]
**cephfs** | Option<[**crate::models::V1CephFsVolumeSource**](v1.CephFSVolumeSource.md)> |  | [optional]
**cinder** | Option<[**crate::models::V1CinderVolumeSource**](v1.CinderVolumeSource.md)> |  | [optional]
**config_map** | Option<[**crate::models::V1ConfigMapVolumeSource**](v1.ConfigMapVolumeSource.md)> |  | [optional]
**csi** | Option<[**crate::models::V1CsiVolumeSource**](v1.CSIVolumeSource.md)> |  | [optional]
**downward_api** | Option<[**crate::models::V1DownwardApiVolumeSource**](v1.DownwardAPIVolumeSource.md)> |  | [optional]
**empty_dir** | Option<[**crate::models::V1EmptyDirVolumeSource**](v1.EmptyDirVolumeSource.md)> |  | [optional]
**ephemeral** | Option<[**crate::models::V1EphemeralVolumeSource**](v1.EphemeralVolumeSource.md)> |  | [optional]
**fc** | Option<[**crate::models::V1FcVolumeSource**](v1.FCVolumeSource.md)> |  | [optional]
**flex_volume** | Option<[**crate::models::V1FlexVolumeSource**](v1.FlexVolumeSource.md)> |  | [optional]
**flocker** | Option<[**crate::models::V1FlockerVolumeSource**](v1.FlockerVolumeSource.md)> |  | [optional]
**gce_persistent_disk** | Option<[**crate::models::V1GcePersistentDiskVolumeSource**](v1.GCEPersistentDiskVolumeSource.md)> |  | [optional]
**git_repo** | Option<[**crate::models::V1GitRepoVolumeSource**](v1.GitRepoVolumeSource.md)> |  | [optional]
**glusterfs** | Option<[**crate::models::V1GlusterfsVolumeSource**](v1.GlusterfsVolumeSource.md)> |  | [optional]
**host_path** | Option<[**crate::models::V1HostPathVolumeSource**](v1.HostPathVolumeSource.md)> |  | [optional]
**iscsi** | Option<[**crate::models::V1IscsiVolumeSource**](v1.ISCSIVolumeSource.md)> |  | [optional]
**name** | **String** | Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names | 
**nfs** | Option<[**crate::models::V1NfsVolumeSource**](v1.NFSVolumeSource.md)> |  | [optional]
**persistent_volume_claim** | Option<[**crate::models::V1PersistentVolumeClaimVolumeSource**](v1.PersistentVolumeClaimVolumeSource.md)> |  | [optional]
**photon_persistent_disk** | Option<[**crate::models::V1PhotonPersistentDiskVolumeSource**](v1.PhotonPersistentDiskVolumeSource.md)> |  | [optional]
**portworx_volume** | Option<[**crate::models::V1PortworxVolumeSource**](v1.PortworxVolumeSource.md)> |  | [optional]
**projected** | Option<[**crate::models::V1ProjectedVolumeSource**](v1.ProjectedVolumeSource.md)> |  | [optional]
**quobyte** | Option<[**crate::models::V1QuobyteVolumeSource**](v1.QuobyteVolumeSource.md)> |  | [optional]
**rbd** | Option<[**crate::models::V1RbdVolumeSource**](v1.RBDVolumeSource.md)> |  | [optional]
**scale_io** | Option<[**crate::models::V1ScaleIoVolumeSource**](v1.ScaleIOVolumeSource.md)> |  | [optional]
**secret** | Option<[**crate::models::V1SecretVolumeSource**](v1.SecretVolumeSource.md)> |  | [optional]
**storageos** | Option<[**crate::models::V1StorageOsVolumeSource**](v1.StorageOSVolumeSource.md)> |  | [optional]
**vsphere_volume** | Option<[**crate::models::V1VsphereVirtualDiskVolumeSource**](v1.VsphereVirtualDiskVolumeSource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


