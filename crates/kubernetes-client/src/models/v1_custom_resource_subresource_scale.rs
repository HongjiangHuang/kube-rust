/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CustomResourceSubresourceScale : CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CustomResourceSubresourceScale {
    /// labelSelectorPath defines the JSON path inside of a custom resource that corresponds to Scale `status.selector`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status` or `.spec`. Must be set to work with HorizontalPodAutoscaler. The field pointed by this JSON path must be a string field (not a complex selector struct) which contains a serialized label selector in string form. More info: https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions#scale-subresource If there is no value under the given path in the custom resource, the `status.selector` value in the `/scale` subresource will default to the empty string.
    #[serde(rename = "labelSelectorPath", skip_serializing_if = "Option::is_none")]
    pub label_selector_path: Option<String>,
    /// specReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `spec.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.spec`. If there is no value under the given path in the custom resource, the `/scale` subresource will return an error on GET.
    #[serde(rename = "specReplicasPath")]
    pub spec_replicas_path: String,
    /// statusReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `status.replicas`. Only JSON paths without the array notation are allowed. Must be a JSON Path under `.status`. If there is no value under the given path in the custom resource, the `status.replicas` value in the `/scale` subresource will default to 0.
    #[serde(rename = "statusReplicasPath")]
    pub status_replicas_path: String,
}

impl V1CustomResourceSubresourceScale {
    /// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
    pub fn new(spec_replicas_path: String, status_replicas_path: String) -> V1CustomResourceSubresourceScale {
        V1CustomResourceSubresourceScale {
            label_selector_path: None,
            spec_replicas_path,
            status_replicas_path,
        }
    }
}

