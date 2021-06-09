# \AppsV1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_namespaced_controller_revision**](AppsV1Api.md#create_namespaced_controller_revision) | **Post** /apis/apps/v1/namespaces/{namespace}/controllerrevisions | 
[**create_namespaced_daemon_set**](AppsV1Api.md#create_namespaced_daemon_set) | **Post** /apis/apps/v1/namespaces/{namespace}/daemonsets | 
[**create_namespaced_deployment**](AppsV1Api.md#create_namespaced_deployment) | **Post** /apis/apps/v1/namespaces/{namespace}/deployments | 
[**create_namespaced_replica_set**](AppsV1Api.md#create_namespaced_replica_set) | **Post** /apis/apps/v1/namespaces/{namespace}/replicasets | 
[**create_namespaced_stateful_set**](AppsV1Api.md#create_namespaced_stateful_set) | **Post** /apis/apps/v1/namespaces/{namespace}/statefulsets | 
[**delete_collection_namespaced_controller_revision**](AppsV1Api.md#delete_collection_namespaced_controller_revision) | **Delete** /apis/apps/v1/namespaces/{namespace}/controllerrevisions | 
[**delete_collection_namespaced_daemon_set**](AppsV1Api.md#delete_collection_namespaced_daemon_set) | **Delete** /apis/apps/v1/namespaces/{namespace}/daemonsets | 
[**delete_collection_namespaced_deployment**](AppsV1Api.md#delete_collection_namespaced_deployment) | **Delete** /apis/apps/v1/namespaces/{namespace}/deployments | 
[**delete_collection_namespaced_replica_set**](AppsV1Api.md#delete_collection_namespaced_replica_set) | **Delete** /apis/apps/v1/namespaces/{namespace}/replicasets | 
[**delete_collection_namespaced_stateful_set**](AppsV1Api.md#delete_collection_namespaced_stateful_set) | **Delete** /apis/apps/v1/namespaces/{namespace}/statefulsets | 
[**delete_namespaced_controller_revision**](AppsV1Api.md#delete_namespaced_controller_revision) | **Delete** /apis/apps/v1/namespaces/{namespace}/controllerrevisions/{name} | 
[**delete_namespaced_daemon_set**](AppsV1Api.md#delete_namespaced_daemon_set) | **Delete** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name} | 
[**delete_namespaced_deployment**](AppsV1Api.md#delete_namespaced_deployment) | **Delete** /apis/apps/v1/namespaces/{namespace}/deployments/{name} | 
[**delete_namespaced_replica_set**](AppsV1Api.md#delete_namespaced_replica_set) | **Delete** /apis/apps/v1/namespaces/{namespace}/replicasets/{name} | 
[**delete_namespaced_stateful_set**](AppsV1Api.md#delete_namespaced_stateful_set) | **Delete** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name} | 
[**get_api_resources**](AppsV1Api.md#get_api_resources) | **Get** /apis/apps/v1/ | 
[**list_controller_revision_for_all_namespaces**](AppsV1Api.md#list_controller_revision_for_all_namespaces) | **Get** /apis/apps/v1/controllerrevisions | 
[**list_daemon_set_for_all_namespaces**](AppsV1Api.md#list_daemon_set_for_all_namespaces) | **Get** /apis/apps/v1/daemonsets | 
[**list_deployment_for_all_namespaces**](AppsV1Api.md#list_deployment_for_all_namespaces) | **Get** /apis/apps/v1/deployments | 
[**list_namespaced_controller_revision**](AppsV1Api.md#list_namespaced_controller_revision) | **Get** /apis/apps/v1/namespaces/{namespace}/controllerrevisions | 
[**list_namespaced_daemon_set**](AppsV1Api.md#list_namespaced_daemon_set) | **Get** /apis/apps/v1/namespaces/{namespace}/daemonsets | 
[**list_namespaced_deployment**](AppsV1Api.md#list_namespaced_deployment) | **Get** /apis/apps/v1/namespaces/{namespace}/deployments | 
[**list_namespaced_replica_set**](AppsV1Api.md#list_namespaced_replica_set) | **Get** /apis/apps/v1/namespaces/{namespace}/replicasets | 
[**list_namespaced_stateful_set**](AppsV1Api.md#list_namespaced_stateful_set) | **Get** /apis/apps/v1/namespaces/{namespace}/statefulsets | 
[**list_replica_set_for_all_namespaces**](AppsV1Api.md#list_replica_set_for_all_namespaces) | **Get** /apis/apps/v1/replicasets | 
[**list_stateful_set_for_all_namespaces**](AppsV1Api.md#list_stateful_set_for_all_namespaces) | **Get** /apis/apps/v1/statefulsets | 
[**patch_namespaced_controller_revision**](AppsV1Api.md#patch_namespaced_controller_revision) | **Patch** /apis/apps/v1/namespaces/{namespace}/controllerrevisions/{name} | 
[**patch_namespaced_daemon_set**](AppsV1Api.md#patch_namespaced_daemon_set) | **Patch** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name} | 
[**patch_namespaced_daemon_set_status**](AppsV1Api.md#patch_namespaced_daemon_set_status) | **Patch** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name}/status | 
[**patch_namespaced_deployment**](AppsV1Api.md#patch_namespaced_deployment) | **Patch** /apis/apps/v1/namespaces/{namespace}/deployments/{name} | 
[**patch_namespaced_deployment_scale**](AppsV1Api.md#patch_namespaced_deployment_scale) | **Patch** /apis/apps/v1/namespaces/{namespace}/deployments/{name}/scale | 
[**patch_namespaced_deployment_status**](AppsV1Api.md#patch_namespaced_deployment_status) | **Patch** /apis/apps/v1/namespaces/{namespace}/deployments/{name}/status | 
[**patch_namespaced_replica_set**](AppsV1Api.md#patch_namespaced_replica_set) | **Patch** /apis/apps/v1/namespaces/{namespace}/replicasets/{name} | 
[**patch_namespaced_replica_set_scale**](AppsV1Api.md#patch_namespaced_replica_set_scale) | **Patch** /apis/apps/v1/namespaces/{namespace}/replicasets/{name}/scale | 
[**patch_namespaced_replica_set_status**](AppsV1Api.md#patch_namespaced_replica_set_status) | **Patch** /apis/apps/v1/namespaces/{namespace}/replicasets/{name}/status | 
[**patch_namespaced_stateful_set**](AppsV1Api.md#patch_namespaced_stateful_set) | **Patch** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name} | 
[**patch_namespaced_stateful_set_scale**](AppsV1Api.md#patch_namespaced_stateful_set_scale) | **Patch** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/scale | 
[**patch_namespaced_stateful_set_status**](AppsV1Api.md#patch_namespaced_stateful_set_status) | **Patch** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/status | 
[**read_namespaced_controller_revision**](AppsV1Api.md#read_namespaced_controller_revision) | **Get** /apis/apps/v1/namespaces/{namespace}/controllerrevisions/{name} | 
[**read_namespaced_daemon_set**](AppsV1Api.md#read_namespaced_daemon_set) | **Get** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name} | 
[**read_namespaced_daemon_set_status**](AppsV1Api.md#read_namespaced_daemon_set_status) | **Get** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name}/status | 
[**read_namespaced_deployment**](AppsV1Api.md#read_namespaced_deployment) | **Get** /apis/apps/v1/namespaces/{namespace}/deployments/{name} | 
[**read_namespaced_deployment_scale**](AppsV1Api.md#read_namespaced_deployment_scale) | **Get** /apis/apps/v1/namespaces/{namespace}/deployments/{name}/scale | 
[**read_namespaced_deployment_status**](AppsV1Api.md#read_namespaced_deployment_status) | **Get** /apis/apps/v1/namespaces/{namespace}/deployments/{name}/status | 
[**read_namespaced_replica_set**](AppsV1Api.md#read_namespaced_replica_set) | **Get** /apis/apps/v1/namespaces/{namespace}/replicasets/{name} | 
[**read_namespaced_replica_set_scale**](AppsV1Api.md#read_namespaced_replica_set_scale) | **Get** /apis/apps/v1/namespaces/{namespace}/replicasets/{name}/scale | 
[**read_namespaced_replica_set_status**](AppsV1Api.md#read_namespaced_replica_set_status) | **Get** /apis/apps/v1/namespaces/{namespace}/replicasets/{name}/status | 
[**read_namespaced_stateful_set**](AppsV1Api.md#read_namespaced_stateful_set) | **Get** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name} | 
[**read_namespaced_stateful_set_scale**](AppsV1Api.md#read_namespaced_stateful_set_scale) | **Get** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/scale | 
[**read_namespaced_stateful_set_status**](AppsV1Api.md#read_namespaced_stateful_set_status) | **Get** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/status | 
[**replace_namespaced_controller_revision**](AppsV1Api.md#replace_namespaced_controller_revision) | **Put** /apis/apps/v1/namespaces/{namespace}/controllerrevisions/{name} | 
[**replace_namespaced_daemon_set**](AppsV1Api.md#replace_namespaced_daemon_set) | **Put** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name} | 
[**replace_namespaced_daemon_set_status**](AppsV1Api.md#replace_namespaced_daemon_set_status) | **Put** /apis/apps/v1/namespaces/{namespace}/daemonsets/{name}/status | 
[**replace_namespaced_deployment**](AppsV1Api.md#replace_namespaced_deployment) | **Put** /apis/apps/v1/namespaces/{namespace}/deployments/{name} | 
[**replace_namespaced_deployment_scale**](AppsV1Api.md#replace_namespaced_deployment_scale) | **Put** /apis/apps/v1/namespaces/{namespace}/deployments/{name}/scale | 
[**replace_namespaced_deployment_status**](AppsV1Api.md#replace_namespaced_deployment_status) | **Put** /apis/apps/v1/namespaces/{namespace}/deployments/{name}/status | 
[**replace_namespaced_replica_set**](AppsV1Api.md#replace_namespaced_replica_set) | **Put** /apis/apps/v1/namespaces/{namespace}/replicasets/{name} | 
[**replace_namespaced_replica_set_scale**](AppsV1Api.md#replace_namespaced_replica_set_scale) | **Put** /apis/apps/v1/namespaces/{namespace}/replicasets/{name}/scale | 
[**replace_namespaced_replica_set_status**](AppsV1Api.md#replace_namespaced_replica_set_status) | **Put** /apis/apps/v1/namespaces/{namespace}/replicasets/{name}/status | 
[**replace_namespaced_stateful_set**](AppsV1Api.md#replace_namespaced_stateful_set) | **Put** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name} | 
[**replace_namespaced_stateful_set_scale**](AppsV1Api.md#replace_namespaced_stateful_set_scale) | **Put** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/scale | 
[**replace_namespaced_stateful_set_status**](AppsV1Api.md#replace_namespaced_stateful_set_status) | **Put** /apis/apps/v1/namespaces/{namespace}/statefulsets/{name}/status | 



## create_namespaced_controller_revision

> crate::models::V1ControllerRevision create_namespaced_controller_revision(namespace, body, pretty, dry_run, field_manager)


create a ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1ControllerRevision**](V1ControllerRevision.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1ControllerRevision**](v1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_daemon_set

> crate::models::V1DaemonSet create_namespaced_daemon_set(namespace, body, pretty, dry_run, field_manager)


create a DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1DaemonSet**](V1DaemonSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_deployment

> crate::models::V1Deployment create_namespaced_deployment(namespace, body, pretty, dry_run, field_manager)


create a Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Deployment**](V1Deployment.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_replica_set

> crate::models::V1ReplicaSet create_namespaced_replica_set(namespace, body, pretty, dry_run, field_manager)


create a ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1ReplicaSet**](V1ReplicaSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespaced_stateful_set

> crate::models::V1StatefulSet create_namespaced_stateful_set(namespace, body, pretty, dry_run, field_manager)


create a StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1StatefulSet**](V1StatefulSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_controller_revision

> crate::models::V1Status delete_collection_namespaced_controller_revision(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, timeout_seconds, body)


delete collection of ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_daemon_set

> crate::models::V1Status delete_collection_namespaced_daemon_set(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, timeout_seconds, body)


delete collection of DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_deployment

> crate::models::V1Status delete_collection_namespaced_deployment(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, timeout_seconds, body)


delete collection of Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_replica_set

> crate::models::V1Status delete_collection_namespaced_replica_set(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, timeout_seconds, body)


delete collection of ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection_namespaced_stateful_set

> crate::models::V1Status delete_collection_namespaced_stateful_set(namespace, pretty, _continue, dry_run, field_selector, grace_period_seconds, label_selector, limit, orphan_dependents, propagation_policy, resource_version, resource_version_match, timeout_seconds, body)


delete collection of StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_controller_revision

> crate::models::V1Status delete_namespaced_controller_revision(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete a ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ControllerRevision | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_daemon_set

> crate::models::V1Status delete_namespaced_daemon_set(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete a DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_deployment

> crate::models::V1Status delete_namespaced_deployment(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete a Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_replica_set

> crate::models::V1Status delete_namespaced_replica_set(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete a ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespaced_stateful_set

> crate::models::V1Status delete_namespaced_stateful_set(name, namespace, pretty, dry_run, grace_period_seconds, orphan_dependents, propagation_policy, body)


delete a StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**grace_period_seconds** | Option<**i32**> | The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. |  |
**orphan_dependents** | Option<**bool**> | Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both. |  |
**propagation_policy** | Option<**String**> | Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground. |  |
**body** | Option<[**V1DeleteOptions**](V1DeleteOptions.md)> |  |  |

### Return type

[**crate::models::V1Status**](v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_resources

> crate::models::V1ApiResourceList get_api_resources()


get available resources

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1ApiResourceList**](v1.APIResourceList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_controller_revision_for_all_namespaces

> crate::models::V1ControllerRevisionList list_controller_revision_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1ControllerRevisionList**](v1.ControllerRevisionList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_daemon_set_for_all_namespaces

> crate::models::V1DaemonSetList list_daemon_set_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1DaemonSetList**](v1.DaemonSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployment_for_all_namespaces

> crate::models::V1DeploymentList list_deployment_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1DeploymentList**](v1.DeploymentList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_controller_revision

> crate::models::V1ControllerRevisionList list_namespaced_controller_revision(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1ControllerRevisionList**](v1.ControllerRevisionList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_daemon_set

> crate::models::V1DaemonSetList list_namespaced_daemon_set(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1DaemonSetList**](v1.DaemonSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_deployment

> crate::models::V1DeploymentList list_namespaced_deployment(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1DeploymentList**](v1.DeploymentList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_replica_set

> crate::models::V1ReplicaSetList list_namespaced_replica_set(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1ReplicaSetList**](v1.ReplicaSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaced_stateful_set

> crate::models::V1StatefulSetList list_namespaced_stateful_set(namespace, pretty, allow_watch_bookmarks, _continue, field_selector, label_selector, limit, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1StatefulSetList**](v1.StatefulSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_replica_set_for_all_namespaces

> crate::models::V1ReplicaSetList list_replica_set_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1ReplicaSetList**](v1.ReplicaSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_stateful_set_for_all_namespaces

> crate::models::V1StatefulSetList list_stateful_set_for_all_namespaces(allow_watch_bookmarks, _continue, field_selector, label_selector, limit, pretty, resource_version, resource_version_match, timeout_seconds, watch)


list or watch objects of kind StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_watch_bookmarks** | Option<**bool**> | allowWatchBookmarks requests watch events with type \"BOOKMARK\". Servers that do not implement bookmarks may ignore this flag and bookmarks are sent at the server's discretion. Clients should not assume bookmarks are returned at any specific interval, nor may they assume the server will send any BOOKMARK event during a session. If this is not a watch, this field is ignored. If the feature gate WatchBookmarks is not enabled in apiserver, this field is ignored. |  |
**_continue** | Option<**String**> | The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server, the server will respond with a 410 ResourceExpired error together with a continue token. If the client needs a consistent list, it must restart their list without the continue field. Otherwise, the client may send another list request with the token received with the 410 error, the server will respond with a list starting from the next key, but from the latest snapshot, which is inconsistent from the previous list results - objects that are created, modified, or deleted after the first list request will be included in the response, as long as their keys are after the \"next key\".  This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications. |  |
**field_selector** | Option<**String**> | A selector to restrict the list of returned objects by their fields. Defaults to everything. |  |
**label_selector** | Option<**String**> | A selector to restrict the list of returned objects by their labels. Defaults to everything. |  |
**limit** | Option<**i32**> | limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.  The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**resource_version** | Option<**String**> | resourceVersion sets a constraint on what resource versions a request may be served from. See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**resource_version_match** | Option<**String**> | resourceVersionMatch determines how resourceVersion is applied to list calls. It is highly recommended that resourceVersionMatch be set for list calls where resourceVersion is set See https://kubernetes.io/docs/reference/using-api/api-concepts/#resource-versions for details.  Defaults to unset |  |
**timeout_seconds** | Option<**i32**> | Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity. |  |
**watch** | Option<**bool**> | Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. |  |

### Return type

[**crate::models::V1StatefulSetList**](v1.StatefulSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_controller_revision

> crate::models::V1ControllerRevision patch_namespaced_controller_revision(name, namespace, body, pretty, dry_run, field_manager, force)


partially update the specified ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ControllerRevision | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1ControllerRevision**](v1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_daemon_set

> crate::models::V1DaemonSet patch_namespaced_daemon_set(name, namespace, body, pretty, dry_run, field_manager, force)


partially update the specified DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_daemon_set_status

> crate::models::V1DaemonSet patch_namespaced_daemon_set_status(name, namespace, body, pretty, dry_run, field_manager, force)


partially update status of the specified DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_deployment

> crate::models::V1Deployment patch_namespaced_deployment(name, namespace, body, pretty, dry_run, field_manager, force)


partially update the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_deployment_scale

> crate::models::V1Scale patch_namespaced_deployment_scale(name, namespace, body, pretty, dry_run, field_manager, force)


partially update scale of the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_deployment_status

> crate::models::V1Deployment patch_namespaced_deployment_status(name, namespace, body, pretty, dry_run, field_manager, force)


partially update status of the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_replica_set

> crate::models::V1ReplicaSet patch_namespaced_replica_set(name, namespace, body, pretty, dry_run, field_manager, force)


partially update the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_replica_set_scale

> crate::models::V1Scale patch_namespaced_replica_set_scale(name, namespace, body, pretty, dry_run, field_manager, force)


partially update scale of the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_replica_set_status

> crate::models::V1ReplicaSet patch_namespaced_replica_set_status(name, namespace, body, pretty, dry_run, field_manager, force)


partially update status of the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_stateful_set

> crate::models::V1StatefulSet patch_namespaced_stateful_set(name, namespace, body, pretty, dry_run, field_manager, force)


partially update the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_stateful_set_scale

> crate::models::V1Scale patch_namespaced_stateful_set_scale(name, namespace, body, pretty, dry_run, field_manager, force)


partially update scale of the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_namespaced_stateful_set_status

> crate::models::V1StatefulSet patch_namespaced_stateful_set_status(name, namespace, body, pretty, dry_run, field_manager, force)


partially update status of the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | **serde_json::Value** |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. This field is required for apply requests (application/apply-patch) but optional for non-apply patch types (JsonPatch, MergePatch, StrategicMergePatch). |  |
**force** | Option<**bool**> | Force is going to \"force\" Apply requests. It means user will re-acquire conflicting fields owned by other people. Force flag must be unset for non-apply patch requests. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json, application/apply-patch+yaml
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_controller_revision

> crate::models::V1ControllerRevision read_namespaced_controller_revision(name, namespace, pretty)


read the specified ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ControllerRevision | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1ControllerRevision**](v1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_daemon_set

> crate::models::V1DaemonSet read_namespaced_daemon_set(name, namespace, pretty)


read the specified DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_daemon_set_status

> crate::models::V1DaemonSet read_namespaced_daemon_set_status(name, namespace, pretty)


read status of the specified DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_deployment

> crate::models::V1Deployment read_namespaced_deployment(name, namespace, pretty)


read the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_deployment_scale

> crate::models::V1Scale read_namespaced_deployment_scale(name, namespace, pretty)


read scale of the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_deployment_status

> crate::models::V1Deployment read_namespaced_deployment_status(name, namespace, pretty)


read status of the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_replica_set

> crate::models::V1ReplicaSet read_namespaced_replica_set(name, namespace, pretty)


read the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_replica_set_scale

> crate::models::V1Scale read_namespaced_replica_set_scale(name, namespace, pretty)


read scale of the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_replica_set_status

> crate::models::V1ReplicaSet read_namespaced_replica_set_status(name, namespace, pretty)


read status of the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_stateful_set

> crate::models::V1StatefulSet read_namespaced_stateful_set(name, namespace, pretty)


read the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_stateful_set_scale

> crate::models::V1Scale read_namespaced_stateful_set_scale(name, namespace, pretty)


read scale of the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_namespaced_stateful_set_status

> crate::models::V1StatefulSet read_namespaced_stateful_set_status(name, namespace, pretty)


read status of the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_controller_revision

> crate::models::V1ControllerRevision replace_namespaced_controller_revision(name, namespace, body, pretty, dry_run, field_manager)


replace the specified ControllerRevision

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ControllerRevision | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1ControllerRevision**](V1ControllerRevision.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1ControllerRevision**](v1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_daemon_set

> crate::models::V1DaemonSet replace_namespaced_daemon_set(name, namespace, body, pretty, dry_run, field_manager)


replace the specified DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1DaemonSet**](V1DaemonSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_daemon_set_status

> crate::models::V1DaemonSet replace_namespaced_daemon_set_status(name, namespace, body, pretty, dry_run, field_manager)


replace status of the specified DaemonSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the DaemonSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1DaemonSet**](V1DaemonSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1DaemonSet**](v1.DaemonSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_deployment

> crate::models::V1Deployment replace_namespaced_deployment(name, namespace, body, pretty, dry_run, field_manager)


replace the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Deployment**](V1Deployment.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_deployment_scale

> crate::models::V1Scale replace_namespaced_deployment_scale(name, namespace, body, pretty, dry_run, field_manager)


replace scale of the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Scale**](V1Scale.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_deployment_status

> crate::models::V1Deployment replace_namespaced_deployment_status(name, namespace, body, pretty, dry_run, field_manager)


replace status of the specified Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Deployment | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Deployment**](V1Deployment.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1Deployment**](v1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_replica_set

> crate::models::V1ReplicaSet replace_namespaced_replica_set(name, namespace, body, pretty, dry_run, field_manager)


replace the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1ReplicaSet**](V1ReplicaSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_replica_set_scale

> crate::models::V1Scale replace_namespaced_replica_set_scale(name, namespace, body, pretty, dry_run, field_manager)


replace scale of the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Scale**](V1Scale.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_replica_set_status

> crate::models::V1ReplicaSet replace_namespaced_replica_set_status(name, namespace, body, pretty, dry_run, field_manager)


replace status of the specified ReplicaSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the ReplicaSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1ReplicaSet**](V1ReplicaSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1ReplicaSet**](v1.ReplicaSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_stateful_set

> crate::models::V1StatefulSet replace_namespaced_stateful_set(name, namespace, body, pretty, dry_run, field_manager)


replace the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1StatefulSet**](V1StatefulSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_stateful_set_scale

> crate::models::V1Scale replace_namespaced_stateful_set_scale(name, namespace, body, pretty, dry_run, field_manager)


replace scale of the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the Scale | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1Scale**](V1Scale.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1Scale**](v1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_namespaced_stateful_set_status

> crate::models::V1StatefulSet replace_namespaced_stateful_set_status(name, namespace, body, pretty, dry_run, field_manager)


replace status of the specified StatefulSet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name of the StatefulSet | [required] |
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1StatefulSet**](V1StatefulSet.md) |  | [required] |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |

### Return type

[**crate::models::V1StatefulSet**](v1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

