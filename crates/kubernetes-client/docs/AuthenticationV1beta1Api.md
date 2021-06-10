# \AuthenticationV1beta1Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_token_review**](AuthenticationV1beta1Api.md#create_token_review) | **POST** /apis/authentication.k8s.io/v1beta1/tokenreviews | 
[**get_api_resources**](AuthenticationV1beta1Api.md#get_api_resources) | **GET** /apis/authentication.k8s.io/v1beta1/ | 



## create_token_review

> crate::models::V1beta1TokenReview create_token_review(body, dry_run, field_manager, pretty)


create a TokenReview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**V1beta1TokenReview**](V1beta1TokenReview.md) |  | [required] |
**dry_run** | Option<**String**> | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |  |
**field_manager** | Option<**String**> | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |  |
**pretty** | Option<**String**> | If 'true', then the output is pretty printed. |  |

### Return type

[**crate::models::V1beta1TokenReview**](v1beta1.TokenReview.md)

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

