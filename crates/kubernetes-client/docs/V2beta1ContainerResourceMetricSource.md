# V2beta1ContainerResourceMetricSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container** | **String** | container is the name of the container in the pods of the scaling target | 
**name** | **String** | name is the name of the resource in question. | 
**target_average_utilization** | Option<**i32**> | targetAverageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. | [optional]
**target_average_value** | Option<**String**> | targetAverageValue is the target value of the average of the resource metric across all relevant pods, as a raw value (instead of as a percentage of the request), similar to the \"pods\" metric source type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


