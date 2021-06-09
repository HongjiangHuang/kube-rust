# V2beta1ExternalMetricSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric_name** | **String** | metricName is the name of the metric in question. | 
**metric_selector** | Option<[**crate::models::V1LabelSelector**](v1.LabelSelector.md)> |  | [optional]
**target_average_value** | Option<**String**> | targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue. | [optional]
**target_value** | Option<**String**> | targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


