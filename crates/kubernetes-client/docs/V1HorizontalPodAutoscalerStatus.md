# V1HorizontalPodAutoscalerStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_cpu_utilization_percentage** | Option<**i32**> | current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU. | [optional]
**current_replicas** | **i32** | current number of replicas of pods managed by this autoscaler. | 
**desired_replicas** | **i32** | desired number of replicas of pods managed by this autoscaler. | 
**last_scale_time** | Option<**String**> | last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed. | [optional]
**observed_generation** | Option<**i64**> | most recent generation observed by this autoscaler. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


