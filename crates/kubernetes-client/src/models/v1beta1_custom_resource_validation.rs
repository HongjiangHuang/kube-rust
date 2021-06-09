/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1CustomResourceValidation : CustomResourceValidation is a list of validation methods for CustomResources.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1CustomResourceValidation {
    #[serde(rename = "openAPIV3Schema", skip_serializing_if = "Option::is_none")]
    pub open_apiv3_schema: Option<crate::models::V1beta1JsonSchemaProps>,
}

impl V1beta1CustomResourceValidation {
    /// CustomResourceValidation is a list of validation methods for CustomResources.
    pub fn new() -> V1beta1CustomResourceValidation {
        V1beta1CustomResourceValidation {
            open_apiv3_schema: None,
        }
    }
}

