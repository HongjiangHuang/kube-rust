/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1TypedLocalObjectReference : TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1TypedLocalObjectReference {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(rename = "apiGroup", skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    #[serde(rename = "kind")]
    pub kind: String,
    /// Name is the name of resource being referenced
    #[serde(rename = "name")]
    pub name: String,
}

impl V1TypedLocalObjectReference {
    /// TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace.
    pub fn new(kind: String, name: String) -> V1TypedLocalObjectReference {
        V1TypedLocalObjectReference {
            api_group: None,
            kind,
            name,
        }
    }
}

