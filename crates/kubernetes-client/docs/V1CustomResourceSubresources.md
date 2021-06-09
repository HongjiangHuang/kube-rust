# V1CustomResourceSubresources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scale** | Option<[**crate::models::V1CustomResourceSubresourceScale**](v1.CustomResourceSubresourceScale.md)> |  | [optional]
**status** | Option<[**serde_json::Value**](.md)> | status indicates the custom resource should serve a `/status` subresource. When enabled: 1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object. 2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


