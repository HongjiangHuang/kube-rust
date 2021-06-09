# V1WebhookConversion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_config** | Option<[**crate::models::ApiextensionsV1WebhookClientConfig**](apiextensions.v1.WebhookClientConfig.md)> |  | [optional]
**conversion_review_versions** | **Vec<String>** | conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


