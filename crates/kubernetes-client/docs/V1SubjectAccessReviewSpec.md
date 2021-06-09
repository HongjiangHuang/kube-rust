# V1SubjectAccessReviewSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here. | [optional]
**groups** | Option<**Vec<String>**> | Groups is the groups you're testing for. | [optional]
**non_resource_attributes** | Option<[**crate::models::V1NonResourceAttributes**](v1.NonResourceAttributes.md)> |  | [optional]
**resource_attributes** | Option<[**crate::models::V1ResourceAttributes**](v1.ResourceAttributes.md)> |  | [optional]
**uid** | Option<**String**> | UID information about the requesting user. | [optional]
**user** | Option<**String**> | User is the user you're testing for. If you specify \"User\" but not \"Groups\", then is it interpreted as \"What if User were not a member of any groups | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


