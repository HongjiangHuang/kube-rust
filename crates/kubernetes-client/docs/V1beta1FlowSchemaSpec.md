# V1beta1FlowSchemaSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**distinguisher_method** | Option<[**crate::models::V1beta1FlowDistinguisherMethod**](v1beta1.FlowDistinguisherMethod.md)> |  | [optional]
**matching_precedence** | Option<**i32**> | `matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be ranged in [1,10000]. Note that if the precedence is not specified, it will be set to 1000 as default. | [optional]
**priority_level_configuration** | [**crate::models::V1beta1PriorityLevelConfigurationReference**](v1beta1.PriorityLevelConfigurationReference.md) |  | 
**rules** | Option<[**Vec<crate::models::V1beta1PolicyRulesWithSubjects>**](v1beta1.PolicyRulesWithSubjects.md)> | `rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


