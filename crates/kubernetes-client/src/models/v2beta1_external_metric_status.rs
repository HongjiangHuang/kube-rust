/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta1ExternalMetricStatus : ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta1ExternalMetricStatus {
    /// currentAverageValue is the current value of metric averaged over autoscaled pods.
    #[serde(rename = "currentAverageValue", skip_serializing_if = "Option::is_none")]
    pub current_average_value: Option<String>,
    /// currentValue is the current value of the metric (as a quantity)
    #[serde(rename = "currentValue")]
    pub current_value: String,
    /// metricName is the name of a metric used for autoscaling in metric system.
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "metricSelector", skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<crate::models::V1LabelSelector>,
}

impl V2beta1ExternalMetricStatus {
    /// ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
    pub fn new(current_value: String, metric_name: String) -> V2beta1ExternalMetricStatus {
        V2beta1ExternalMetricStatus {
            current_average_value: None,
            current_value,
            metric_name,
            metric_selector: None,
        }
    }
}

