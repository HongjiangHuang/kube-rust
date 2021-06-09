# V1beta1CertificateSigningRequestCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_transition_time** | Option<**String**> | lastTransitionTime is the time the condition last transitioned from one status to another. If unset, when a new condition type is added or an existing condition's status is changed, the server defaults this to the current time. | [optional]
**last_update_time** | Option<**String**> | timestamp for the last update to this condition | [optional]
**message** | Option<**String**> | human readable message with details about the request state | [optional]
**reason** | Option<**String**> | brief reason for the request state | [optional]
**status** | Option<**String**> | Status of the condition, one of True, False, Unknown. Approved, Denied, and Failed conditions may not be \"False\" or \"Unknown\". Defaults to \"True\". If unset, should be treated as \"True\". | [optional]
**_type** | **String** | type of the condition. Known conditions include \"Approved\", \"Denied\", and \"Failed\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


