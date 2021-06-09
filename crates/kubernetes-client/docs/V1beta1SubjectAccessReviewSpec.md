# V1beta1SubjectAccessReviewSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here. | [optional]
**group** | Option<**Vec<String>**> | Groups is the groups you're testing for. | [optional]
**non_resource_attributes** | Option<[**crate::models::V1beta1NonResourceAttributes**](v1beta1.NonResourceAttributes.md)> |  | [optional]
**resource_attributes** | Option<[**crate::models::V1beta1ResourceAttributes**](v1beta1.ResourceAttributes.md)> |  | [optional]
**uid** | Option<**String**> | UID information about the requesting user. | [optional]
**user** | Option<**String**> | User is the user you're testing for. If you specify \"User\" but not \"Group\", then is it interpreted as \"What if User were not a member of any groups | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


