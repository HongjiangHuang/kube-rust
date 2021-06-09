# V2beta1ContainerResourceMetricStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container** | **String** | container is the name of the container in the pods of the scaling target | 
**current_average_utilization** | Option<**i32**> | currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.  It will only be present if `targetAverageValue` was set in the corresponding metric specification. | [optional]
**current_average_value** | **String** | currentAverageValue is the current value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the \"pods\" metric source type. It will always be set, regardless of the corresponding metric specification. | 
**name** | **String** | name is the name of the resource in question. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


