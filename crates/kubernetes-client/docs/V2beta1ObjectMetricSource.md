# V2beta1ObjectMetricSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_value** | Option<**String**> | averageValue is the target value of the average of the metric across all relevant pods (as a quantity) | [optional]
**metric_name** | **String** | metricName is the name of the metric in question. | 
**selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]
**target** | [**crate::models::V2beta1CrossVersionObjectReference**](v2beta1.CrossVersionObjectReference.md) |  | 
**target_value** | **String** | targetValue is the target value of the metric (as a quantity). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


