# V2beta1ObjectMetricStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_value** | Option<**String**> | averageValue is the current value of the average of the metric across all relevant pods (as a quantity) | [optional]
**current_value** | **String** | currentValue is the current value of the metric (as a quantity). | 
**metric_name** | **String** | metricName is the name of the metric in question. | 
**selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]
**target** | [**crate::models::V2beta1CrossVersionObjectReference**](v2beta1.CrossVersionObjectReference.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


