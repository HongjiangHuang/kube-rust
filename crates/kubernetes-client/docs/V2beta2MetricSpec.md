# V2beta2MetricSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_resource** | Option<[**crate::models::V2beta2ContainerResourceMetricSource**](v2beta2.ContainerResourceMetricSource.md)> |  | [optional]
**external** | Option<[**crate::models::V2beta2ExternalMetricSource**](v2beta2.ExternalMetricSource.md)> |  | [optional]
**object** | Option<[**crate::models::V2beta2ObjectMetricSource**](v2beta2.ObjectMetricSource.md)> |  | [optional]
**pods** | Option<[**crate::models::V2beta2PodsMetricSource**](v2beta2.PodsMetricSource.md)> |  | [optional]
**resource** | Option<[**crate::models::V2beta2ResourceMetricSource**](v2beta2.ResourceMetricSource.md)> |  | [optional]
**_type** | **String** | type is the type of metric source.  It should be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each mapping to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


