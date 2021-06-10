use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod admissionregistration_api;
pub mod admissionregistration_v1_api;
pub mod admissionregistration_v1beta1_api;
pub mod apiextensions_api;
pub mod apiextensions_v1_api;
pub mod apiextensions_v1beta1_api;
pub mod apiregistration_api;
pub mod apiregistration_v1_api;
pub mod apiregistration_v1beta1_api;
pub mod apis_api;
pub mod apps_api;
pub mod apps_v1_api;
pub mod authentication_api;
pub mod authentication_v1_api;
pub mod authentication_v1beta1_api;
pub mod authorization_api;
pub mod authorization_v1_api;
pub mod authorization_v1beta1_api;
pub mod autoscaling_api;
pub mod autoscaling_v1_api;
pub mod autoscaling_v2beta1_api;
pub mod autoscaling_v2beta2_api;
pub mod batch_api;
pub mod batch_v1_api;
pub mod batch_v1beta1_api;
pub mod certificates_api;
pub mod certificates_v1_api;
pub mod certificates_v1beta1_api;
pub mod coordination_api;
pub mod coordination_v1_api;
pub mod coordination_v1beta1_api;
pub mod core_api;
pub mod core_v1_api;
pub mod custom_objects_api;
pub mod discovery_api;
pub mod discovery_v1_api;
pub mod discovery_v1beta1_api;
pub mod events_api;
pub mod events_v1_api;
pub mod events_v1beta1_api;
pub mod extensions_api;
pub mod extensions_v1beta1_api;
pub mod flowcontrol_apiserver_api;
pub mod flowcontrol_apiserver_v1beta1_api;
pub mod internal_apiserver_api;
pub mod internal_apiserver_v1alpha1_api;
pub mod logs_api;
pub mod networking_api;
pub mod networking_v1_api;
pub mod networking_v1beta1_api;
pub mod node_api;
pub mod node_v1_api;
pub mod node_v1alpha1_api;
pub mod node_v1beta1_api;
pub mod openid_api;
pub mod policy_api;
pub mod policy_v1_api;
pub mod policy_v1beta1_api;
pub mod rbac_authorization_api;
pub mod rbac_authorization_v1_api;
pub mod rbac_authorization_v1alpha1_api;
pub mod rbac_authorization_v1beta1_api;
pub mod scheduling_api;
pub mod scheduling_v1_api;
pub mod scheduling_v1alpha1_api;
pub mod scheduling_v1beta1_api;
pub mod storage_api;
pub mod storage_v1_api;
pub mod storage_v1alpha1_api;
pub mod storage_v1beta1_api;
pub mod version_api;
pub mod well_known_api;

pub mod configuration;
