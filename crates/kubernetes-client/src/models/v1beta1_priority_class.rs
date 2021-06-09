/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1PriorityClass : DEPRECATED - This group version of PriorityClass is deprecated by scheduling.k8s.io/v1/PriorityClass. PriorityClass defines mapping from a priority class name to the priority integer value. The value can be any valid integer.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1PriorityClass {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// description is an arbitrary string that usually provides guidelines on when this priority class should be used.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// globalDefault specifies whether this PriorityClass should be considered as the default priority for pods that do not have any priority class. Only one PriorityClass can be marked as `globalDefault`. However, if more than one PriorityClasses exists with their `globalDefault` field set to true, the smallest value of such global default PriorityClasses will be used as the default priority.
    #[serde(rename = "globalDefault", skip_serializing_if = "Option::is_none")]
    pub global_default: Option<bool>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::V1ObjectMeta>,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. This field is beta-level, gated by the NonPreemptingPriority feature-gate.
    #[serde(rename = "preemptionPolicy", skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<String>,
    /// The value of this priority class. This is the actual priority that pods receive when they have the name of this class in their pod spec.
    #[serde(rename = "value")]
    pub value: i32,
}

impl V1beta1PriorityClass {
    /// DEPRECATED - This group version of PriorityClass is deprecated by scheduling.k8s.io/v1/PriorityClass. PriorityClass defines mapping from a priority class name to the priority integer value. The value can be any valid integer.
    pub fn new(value: i32) -> V1beta1PriorityClass {
        V1beta1PriorityClass {
            api_version: None,
            description: None,
            global_default: None,
            kind: None,
            metadata: None,
            preemption_policy: None,
            value,
        }
    }
}

