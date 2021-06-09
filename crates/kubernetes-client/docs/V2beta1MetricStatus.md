# V2beta1MetricStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_resource** | Option<[**crate::models::V2beta1ContainerResourceMetricStatus**](v2beta1.ContainerResourceMetricStatus.md)> |  | [optional]
**external** | Option<[**crate::models::V2beta1ExternalMetricStatus**](v2beta1.ExternalMetricStatus.md)> |  | [optional]
**object** | Option<[**crate::models::V2beta1ObjectMetricStatus**](v2beta1.ObjectMetricStatus.md)> |  | [optional]
**pods** | Option<[**crate::models::V2beta1PodsMetricStatus**](v2beta1.PodsMetricStatus.md)> |  | [optional]
**resource** | Option<[**crate::models::V2beta1ResourceMetricStatus**](v2beta1.ResourceMetricStatus.md)> |  | [optional]
**_type** | **String** | type is the type of metric source.  It will be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each corresponds to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


