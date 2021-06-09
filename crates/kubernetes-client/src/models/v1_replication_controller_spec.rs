/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ReplicationControllerSpec : ReplicationControllerSpec is the specification of a replication controller.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ReplicationControllerSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(rename = "minReadySeconds", skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Selector is a label query over pods that should match the Replicas count. If Selector is empty, it is defaulted to the labels present on the Pod template. Label keys and values that must match in order to be controlled by this replication controller, if empty defaulted to labels on Pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<crate::models::V1PodTemplateSpec>,
}

impl V1ReplicationControllerSpec {
    /// ReplicationControllerSpec is the specification of a replication controller.
    pub fn new() -> V1ReplicationControllerSpec {
        V1ReplicationControllerSpec {
            min_ready_seconds: None,
            replicas: None,
            selector: None,
            template: None,
        }
    }
}

