# \CustomObjectsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster_custom_object**](CustomObjectsApi.md#create_cluster_custom_object) | **POST** /apis/{group}/{version}/{plural} | 
[**create_namespaced_custom_object**](CustomObjectsApi.md#create_namespaced_custom_object) | **POST** /apis/{group}/{version}/namespaces/{namespace}/{plural} | 
[**delete_cluster_custom_object**](CustomObjectsApi.md#delete_cluster_custom_object) | **DELETE** /apis/{group}/{version}/{plural}/{name} | 
[**delete_collection_cluster_custom_object**](CustomObjectsApi.md#delete_collection_cluster_custom_object) | **DELETE** /apis/{group}/{version}/{plural} | 
[**delete_collection_namespaced_custom_object**](CustomObjectsApi.md#delete_collection_namespaced_custom_object) | **DELETE** /apis/{group}/{version}/namespaces/{namespace}/{plural} | 
[**delete_namespaced_custom_object**](CustomObjectsApi.md#delete_namespaced_custom_object) | **DELETE** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**get_cluster_custom_object**](CustomObjectsApi.md#get_cluster_custom_object) | **GET** /apis/{group}/{version}/{plural}/{name} | 
[**get_cluster_custom_object_scale**](CustomObjectsApi.md#get_cluster_custom_object_scale) | **GET** /apis/{group}/{version}/{plural}/{name}/scale | 
[**get_cluster_custom_object_status**](CustomObjectsApi.md#get_cluster_custom_object_status) | **GET** /apis/{group}/{version}/{plural}/{name}/status | 
[**get_namespaced_custom_object**](CustomObjectsApi.md#get_namespaced_custom_object) | **GET** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**get_namespaced_custom_object_scale**](CustomObjectsApi.md#get_namespaced_custom_object_scale) | **GET** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/scale | 
[**get_namespaced_custom_object_status**](CustomObjectsApi.md#get_namespaced_custom_object_status) | **GET** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/status | 
[**list_cluster_custom_object**](CustomObjectsApi.md#list_cluster_custom_object) | **GET** /apis/{group}/{version}/{plural} | 
[**list_namespaced_custom_object**](CustomObjectsApi.md#list_namespaced_custom_object) | **GET** /apis/{group}/{version}/namespaces/{namespace}/{plural} | 
[**patch_cluster_custom_object**](CustomObjectsApi.md#patch_cluster_custom_object) | **PATCH** /apis/{group}/{version}/{plural}/{name} | 
[**patch_cluster_custom_object_scale**](CustomObjectsApi.md#patch_cluster_custom_object_scale) | **PATCH** /apis/{group}/{version}/{plural}/{name}/scale | 
[**patch_cluster_custom_object_status**](CustomObjectsApi.md#patch_cluster_custom_object_status) | **PATCH** /apis/{group}/{version}/{plural}/{name}/status | 
[**patch_namespaced_custom_object**](CustomObjectsApi.md#patch_namespaced_custom_object) | **PATCH** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**patch_namespaced_custom_object_scale**](CustomObjectsApi.md#patch_namespaced_custom_object_scale) | **PATCH** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/scale | 
[**patch_namespaced_custom_object_status**](CustomObjectsApi.md#patch_namespaced_custom_object_status) | **PATCH** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/status | 
[**replace_cluster_custom_object**](CustomObjectsApi.md#replace_cluster_custom_object) | **PUT** /apis/{group}/{version}/{plural}/{name} | 
[**replace_cluster_custom_object_scale**](CustomObjectsApi.md#replace_cluster_custom_object_scale) | **PUT** /apis/{group}/{version}/{plural}/{name}/scale | 
[**replace_cluster_custom_object_status**](CustomObjectsApi.md#replace_cluster_custom_object_status) | **PUT** /apis/{group}/{version}/{plural}/{name}/status | 
[**replace_namespaced_custom_object**](CustomObjectsApi.md#replace_namespaced_custom_object) | **PUT** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name} | 
[**replace_namespaced_custom_object_scale**](CustomObjectsApi.md#replace_namespaced_custom_object_scale) | **PUT** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/scale | 
[**replace_namespaced_custom_object_status**](CustomObjectsApi.md#replace_namespaced_custom_object_status) | **PUT** /apis/{group}/{version}/namespaces/{namespace}/{plural}/{name}/status | 



## create_cluster_custom_object

> serde_json::Value create_cluster_custom_object(group, version, plural, body, pretty, dry_run, field_manager)


Creates a cluster scoped Custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | The custom resource's group name | [required] |
**version** | **String** | The custom resource's version | [required] |
**plural** | **String** | The custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**body** | **serde_json::Value** | The JSON schema of the Resource to create. | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_custom_object

> serde_json::Value create_namespaced_custom_object(group, version, namespace, plural, body, pretty, dry_run, field_manager)


Creates a namespace scoped Custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | The custom resource's group name | [required] |
**version** | **String** | The custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | The custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**body** | **serde_json::Value** | The JSON schema of the Resource to create. | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster_custom_object

> serde_json::Value delete_cluster_custom_object(group, version, plural, name, grace_period_seconds, orphan_dependents, propagation_policy, dry_run, body)


Deletes the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom object's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_cluster_custom_object

> serde_json::Value delete_collection_cluster_custom_object(group, version, plural, pretty, grace_period_seconds, orphan_dependents, propagation_policy, dry_run, body)


Delete collection of cluster scoped custom objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | The custom resource's group name | [required] |
**version** | **String** | The custom resource's version | [required] |
**plural** | **String** | The custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_custom_object

> serde_json::Value delete_collection_namespaced_custom_object(group, version, namespace, plural, pretty, grace_period_seconds, orphan_dependents, propagation_policy, dry_run, body)


Delete collection of namespace scoped custom objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | The custom resource's group name | [required] |
**version** | **String** | The custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | The custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_custom_object

> serde_json::Value delete_namespaced_custom_object(group, version, namespace, plural, name, grace_period_seconds, orphan_dependents, propagation_policy, dry_run, body)


Deletes the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_custom_object

> serde_json::Value get_cluster_custom_object(group, version, plural, name)


Returns a cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom object's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_custom_object_scale

> serde_json::Value get_cluster_custom_object_scale(group, version, plural, name)


read scale of the specified custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster_custom_object_status

> serde_json::Value get_cluster_custom_object_status(group, version, plural, name)


read status of the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespaced_custom_object

> serde_json::Value get_namespaced_custom_object(group, version, namespace, plural, name)


Returns a namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespaced_custom_object_scale

> serde_json::Value get_namespaced_custom_object_scale(group, version, namespace, plural, name)


read scale of the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespaced_custom_object_status

> serde_json::Value get_namespaced_custom_object_status(group, version, namespace, plural, name)


read status of the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_custom_object

> serde_json::Value list_cluster_custom_object(group, version, plural, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch cluster scoped custom objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | The custom resource's group name | [required] |
**version** | **String** | The custom resource's version | [required] |
**plural** | **String** | The custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_custom_object

> serde_json::Value list_namespaced_custom_object(group, version, namespace, plural, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch namespace scoped custom objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | The custom resource's group name | [required] |
**version** | **String** | The custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | The custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_cluster_custom_object

> serde_json::Value patch_cluster_custom_object(group, version, plural, name, body, dry_run, field_manager, force)


patch the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom object's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** | The JSON schema of the Resource to patch. | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_cluster_custom_object_scale

> serde_json::Value patch_cluster_custom_object_scale(group, version, plural, name, body, dry_run, field_manager, force)


partially update scale of the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_cluster_custom_object_status

> serde_json::Value patch_cluster_custom_object_status(group, version, plural, name, body, dry_run, field_manager, force)


partially update status of the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_custom_object

> serde_json::Value patch_namespaced_custom_object(group, version, namespace, plural, name, body, dry_run, field_manager, force)


patch the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** | The JSON schema of the Resource to patch. | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_custom_object_scale

> serde_json::Value patch_namespaced_custom_object_scale(group, version, namespace, plural, name, body, dry_run, field_manager, force)


partially update scale of the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_custom_object_status

> serde_json::Value patch_namespaced_custom_object_status(group, version, namespace, plural, name, body, dry_run, field_manager, force)


partially update status of the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_cluster_custom_object

> serde_json::Value replace_cluster_custom_object(group, version, plural, name, body, dry_run, field_manager)


replace the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom object's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** | The JSON schema of the Resource to replace. | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_cluster_custom_object_scale

> serde_json::Value replace_cluster_custom_object_scale(group, version, plural, name, body, dry_run, field_manager)


replace scale of the specified cluster scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_cluster_custom_object_status

> serde_json::Value replace_cluster_custom_object_status(group, version, plural, name, body, dry_run, field_manager)


replace status of the cluster scoped specified custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_custom_object

> serde_json::Value replace_namespaced_custom_object(group, version, namespace, plural, name, body, dry_run, field_manager)


replace the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** | The JSON schema of the Resource to replace. | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_custom_object_scale

> serde_json::Value replace_namespaced_custom_object_scale(group, version, namespace, plural, name, body, dry_run, field_manager)


replace scale of the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_custom_object_status

> serde_json::Value replace_namespaced_custom_object_status(group, version, namespace, plural, name, body, dry_run, field_manager)


replace status of the specified namespace scoped custom object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | the custom resource's group | [required] |
**version** | **String** | the custom resource's version | [required] |
**namespace** | **String** | The custom resource's namespace | [required] |
**plural** | **String** | the custom resource's plural name. For TPRs this would be lowercase plural kind. | [required] |
**name** | **String** | the custom object's name | [required] |
**body** | **serde_json::Value** |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

