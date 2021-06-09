# V2beta2MetricStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_resource** | Option<[**crate::models::V2beta2ContainerResourceMetricStatus**](v2beta2.ContainerResourceMetricStatus.md)> |  | [optional]
**external** | Option<[**crate::models::V2beta2ExternalMetricStatus**](v2beta2.ExternalMetricStatus.md)> |  | [optional]
**object** | Option<[**crate::models::V2beta2ObjectMetricStatus**](v2beta2.ObjectMetricStatus.md)> |  | [optional]
**pods** | Option<[**crate::models::V2beta2PodsMetricStatus**](v2beta2.PodsMetricStatus.md)> |  | [optional]
**resource** | Option<[**crate::models::V2beta2ResourceMetricStatus**](v2beta2.ResourceMetricStatus.md)> |  | [optional]
**_type** | **String** | type is the type of metric source.  It will be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each corresponds to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


