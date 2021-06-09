/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1VolumeAttachmentSpec : VolumeAttachmentSpec is the specification of a VolumeAttachment request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1alpha1VolumeAttachmentSpec {
    /// Attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
    #[serde(rename = "attacher")]
    pub attacher: String,
    /// The node that the volume should be attached to.
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "source")]
    pub source: crate::models::V1alpha1VolumeAttachmentSource,
}

impl V1alpha1VolumeAttachmentSpec {
    /// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
    pub fn new(attacher: String, node_name: String, source: crate::models::V1alpha1VolumeAttachmentSource) -> V1alpha1VolumeAttachmentSpec {
        V1alpha1VolumeAttachmentSpec {
            attacher,
            node_name,
            source,
        }
    }
}

