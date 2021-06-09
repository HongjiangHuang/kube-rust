# V1TokenRequestSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audiences** | **Vec<String>** | Audiences are the intendend audiences of the token. A recipient of a token must identitfy themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences. | 
**bound_object_ref** | Option<[**crate::models::V1BoundObjectReference**](v1.BoundObjectReference.md)> |  | [optional]
**expiration_seconds** | Option<**i64**> | ExpirationSeconds is the requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the 'expiration' field in a response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


