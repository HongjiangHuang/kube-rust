# V1ReplicaSetStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_replicas** | Option<**i32**> | The number of available replicas (ready for at least minReadySeconds) for this replica set. | [optional]
**conditions** | Option<[**Vec<crate::models::V1ReplicaSetCondition>**](v1.ReplicaSetCondition.md)> | Represents the latest available observations of a replica set's current state. | [optional]
**fully_labeled_replicas** | Option<**i32**> | The number of pods that have labels matching the labels of the pod template of the replicaset. | [optional]
**observed_generation** | Option<**i64**> | ObservedGeneration reflects the generation of the most recently observed ReplicaSet. | [optional]
**ready_replicas** | Option<**i32**> | The number of ready replicas for this replica set. | [optional]
**replicas** | **i32** | Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


