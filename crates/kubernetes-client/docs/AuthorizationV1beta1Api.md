# \AuthorizationV1beta1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_namespaced_local_subject_access_review**](AuthorizationV1beta1Api.md#create_namespaced_local_subject_access_review) | **POST** /apis/authorization.k8s.io/v1beta1/namespaces/{namespace}/localsubjectaccessreviews | 
[**create_self_subject_access_review**](AuthorizationV1beta1Api.md#create_self_subject_access_review) | **POST** /apis/authorization.k8s.io/v1beta1/selfsubjectaccessreviews | 
[**create_self_subject_rules_review**](AuthorizationV1beta1Api.md#create_self_subject_rules_review) | **POST** /apis/authorization.k8s.io/v1beta1/selfsubjectrulesreviews | 
[**create_subject_access_review**](AuthorizationV1beta1Api.md#create_subject_access_review) | **POST** /apis/authorization.k8s.io/v1beta1/subjectaccessreviews | 
[**get_api_resources**](AuthorizationV1beta1Api.md#get_api_resources) | **GET** /apis/authorization.k8s.io/v1beta1/ | 



## create_namespaced_local_subject_access_review

> crate::models::V1beta1LocalSubjectAccessReview create_namespaced_local_subject_access_review(namespace, body, dry_run, field_manager, pretty)


create a LocalSubjectAccessReview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** | object name and auth scope, such as for teams and projects | [required] |
**body** | [**V1beta1LocalSubjectAccessReview**](V1beta1LocalSubjectAccessReview.md) |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1beta1LocalSubjectAccessReview**](v1beta1.LocalSubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_self_subject_access_review

> crate::models::V1beta1SelfSubjectAccessReview create_self_subject_access_review(body, dry_run, field_manager, pretty)


create a SelfSubjectAccessReview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1beta1SelfSubjectAccessReview**](V1beta1SelfSubjectAccessReview.md) |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1beta1SelfSubjectAccessReview**](v1beta1.SelfSubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_self_subject_rules_review

> crate::models::V1beta1SelfSubjectRulesReview create_self_subject_rules_review(body, dry_run, field_manager, pretty)


create a SelfSubjectRulesReview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1beta1SelfSubjectRulesReview**](V1beta1SelfSubjectRulesReview.md) |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1beta1SelfSubjectRulesReview**](v1beta1.SelfSubjectRulesReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subject_access_review

> crate::models::V1beta1SubjectAccessReview create_subject_access_review(body, dry_run, field_manager, pretty)


create a SubjectAccessReview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1beta1SubjectAccessReview**](V1beta1SubjectAccessReview.md) |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1beta1SubjectAccessReview**](v1beta1.SubjectAccessReview.md)

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

