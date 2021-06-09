# V1beta1ResourceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group** | Option<**String**> | Group is the API Group of the Resource.  \"*\" means all. | [optional]
**name** | Option<**String**> | Name is the name of the resource being requested for a \"get\" or deleted for a \"delete\". \"\" (empty) means all. | [optional]
**namespace** | Option<**String**> | Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \"\" (empty) is defaulted for LocalSubjectAccessReviews \"\" (empty) is empty for cluster-scoped resources \"\" (empty) means \"all\" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview | [optional]
**resource** | Option<**String**> | Resource is one of the existing resource types.  \"*\" means all. | [optional]
**subresource** | Option<**String**> | Subresource is one of the existing resource types.  \"\" means none. | [optional]
**verb** | Option<**String**> | Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \"*\" means all. | [optional]
**version** | Option<**String**> | Version is the API Version of the Resource.  \"*\" means all. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


