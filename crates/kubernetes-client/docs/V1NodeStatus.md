# V1NodeStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | Option<[**Vec<crate::models::V1NodeAddress>**](v1.NodeAddress.md)> | List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See http://pr.k8s.io/79391 for an example. | [optional]
**allocatable** | Option<**::std::collections::HashMap<String, String>**> | Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity. | [optional]
**capacity** | Option<**::std::collections::HashMap<String, String>**> | Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity | [optional]
**conditions** | Option<[**Vec<crate::models::V1NodeCondition>**](v1.NodeCondition.md)> | Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition | [optional]
**config** | Option<[**crate::models::V1NodeConfigStatus**](v1.NodeConfigStatus.md)> |  | [optional]
**daemon_endpoints** | Option<[**crate::models::V1NodeDaemonEndpoints**](v1.NodeDaemonEndpoints.md)> |  | [optional]
**images** | Option<[**Vec<crate::models::V1ContainerImage>**](v1.ContainerImage.md)> | List of container images on this node | [optional]
**node_info** | Option<[**crate::models::V1NodeSystemInfo**](v1.NodeSystemInfo.md)> |  | [optional]
**phase** | Option<**String**> | NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated. | [optional]
**volumes_attached** | Option<[**Vec<crate::models::V1AttachedVolume>**](v1.AttachedVolume.md)> | List of volumes that are attached to the node. | [optional]
**volumes_in_use** | Option<**Vec<String>**> | List of attachable volumes in use (mounted) by the node. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


