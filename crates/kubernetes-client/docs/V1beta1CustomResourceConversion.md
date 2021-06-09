# V1beta1CustomResourceConversion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversion_review_versions** | Option<**Vec<String>**> | conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail. Defaults to `[\"v1beta1\"]`. | [optional]
**strategy** | **String** | strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information   is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhookClientConfig to be set. | 
**webhook_client_config** | Option<[**crate::models::ApiextensionsV1beta1WebhookClientConfig**](apiextensions.v1beta1.WebhookClientConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


