/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ScaleStatus : ScaleStatus represents the current status of a scale subresource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ScaleStatus {
    /// actual number of observed instances of the scaled object.
    #[serde(rename = "replicas")]
    pub replicas: i32,
    /// label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

impl V1ScaleStatus {
    /// ScaleStatus represents the current status of a scale subresource.
    pub fn new(replicas: i32) -> V1ScaleStatus {
        V1ScaleStatus {
            replicas,
            selector: None,
        }
    }
}

