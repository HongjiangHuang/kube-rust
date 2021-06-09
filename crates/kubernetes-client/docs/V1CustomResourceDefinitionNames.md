# V1CustomResourceDefinitionNames

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | Option<**Vec<String>**> | categories is a list of grouped resources this custom resource belongs to (e.g. 'all'). This is published in API discovery documents, and used by clients to support invocations like `kubectl get all`. | [optional]
**kind** | **String** | kind is the serialized kind of the resource. It is normally CamelCase and singular. Custom resource instances will use this value as the `kind` attribute in API calls. | 
**list_kind** | Option<**String**> | listKind is the serialized kind of the list for this resource. Defaults to \"`kind`List\". | [optional]
**plural** | **String** | plural is the plural name of the resource to serve. The custom resources are served under `/apis/<group>/<version>/.../<plural>`. Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`). Must be all lowercase. | 
**short_names** | Option<**Vec<String>**> | shortNames are short names for the resource, exposed in API discovery documents, and used by clients to support invocations like `kubectl get <shortname>`. It must be all lowercase. | [optional]
**singular** | Option<**String**> | singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


