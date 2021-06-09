# V1beta1CustomResourceDefinitionVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_printer_columns** | Option<[**Vec<crate::models::V1beta1CustomResourceColumnDefinition>**](v1beta1.CustomResourceColumnDefinition.md)> | additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. Top-level and per-version columns are mutually exclusive. Per-version columns must not all be set to identical values (top-level columns should be used instead). If no top-level or per-version columns are specified, a single column displaying the age of the custom resource is used. | [optional]
**deprecated** | Option<**bool**> | deprecated indicates this version of the custom resource API is deprecated. When set to true, API requests to this version receive a warning header in the server response. Defaults to false. | [optional]
**deprecation_warning** | Option<**String**> | deprecationWarning overrides the default warning returned to API clients. May only be set when `deprecated` is true. The default warning indicates this version is deprecated and recommends use of the newest served version of equal or greater stability, if one exists. | [optional]
**name** | **String** | name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true. | 
**schema** | Option<[**crate::models::V1beta1CustomResourceValidation**](v1beta1.CustomResourceValidation.md)> |  | [optional]
**served** | **bool** | served is a flag enabling/disabling this version from being served via REST APIs | 
**storage** | **bool** | storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true. | 
**subresources** | Option<[**crate::models::V1beta1CustomResourceSubresources**](v1beta1.CustomResourceSubresources.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


