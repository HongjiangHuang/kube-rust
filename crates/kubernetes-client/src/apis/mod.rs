use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod admissionregistration_api;
pub use self::admissionregistration_api::{ AdmissionregistrationApi, AdmissionregistrationApiClient };
mod admissionregistration_v1_api;
pub use self::admissionregistration_v1_api::{ AdmissionregistrationV1Api, AdmissionregistrationV1ApiClient };
mod admissionregistration_v1beta1_api;
pub use self::admissionregistration_v1beta1_api::{ AdmissionregistrationV1beta1Api, AdmissionregistrationV1beta1ApiClient };
mod apiextensions_api;
pub use self::apiextensions_api::{ ApiextensionsApi, ApiextensionsApiClient };
mod apiextensions_v1_api;
pub use self::apiextensions_v1_api::{ ApiextensionsV1Api, ApiextensionsV1ApiClient };
mod apiextensions_v1beta1_api;
pub use self::apiextensions_v1beta1_api::{ ApiextensionsV1beta1Api, ApiextensionsV1beta1ApiClient };
mod apiregistration_api;
pub use self::apiregistration_api::{ ApiregistrationApi, ApiregistrationApiClient };
mod apiregistration_v1_api;
pub use self::apiregistration_v1_api::{ ApiregistrationV1Api, ApiregistrationV1ApiClient };
mod apiregistration_v1beta1_api;
pub use self::apiregistration_v1beta1_api::{ ApiregistrationV1beta1Api, ApiregistrationV1beta1ApiClient };
mod apis_api;
pub use self::apis_api::{ ApisApi, ApisApiClient };
mod apps_api;
pub use self::apps_api::{ AppsApi, AppsApiClient };
mod apps_v1_api;
pub use self::apps_v1_api::{ AppsV1Api, AppsV1ApiClient };
mod authentication_api;
pub use self::authentication_api::{ AuthenticationApi, AuthenticationApiClient };
mod authentication_v1_api;
pub use self::authentication_v1_api::{ AuthenticationV1Api, AuthenticationV1ApiClient };
mod authentication_v1beta1_api;
pub use self::authentication_v1beta1_api::{ AuthenticationV1beta1Api, AuthenticationV1beta1ApiClient };
mod authorization_api;
pub use self::authorization_api::{ AuthorizationApi, AuthorizationApiClient };
mod authorization_v1_api;
pub use self::authorization_v1_api::{ AuthorizationV1Api, AuthorizationV1ApiClient };
mod authorization_v1beta1_api;
pub use self::authorization_v1beta1_api::{ AuthorizationV1beta1Api, AuthorizationV1beta1ApiClient };
mod autoscaling_api;
pub use self::autoscaling_api::{ AutoscalingApi, AutoscalingApiClient };
mod autoscaling_v1_api;
pub use self::autoscaling_v1_api::{ AutoscalingV1Api, AutoscalingV1ApiClient };
mod autoscaling_v2beta1_api;
pub use self::autoscaling_v2beta1_api::{ AutoscalingV2beta1Api, AutoscalingV2beta1ApiClient };
mod autoscaling_v2beta2_api;
pub use self::autoscaling_v2beta2_api::{ AutoscalingV2beta2Api, AutoscalingV2beta2ApiClient };
mod batch_api;
pub use self::batch_api::{ BatchApi, BatchApiClient };
mod batch_v1_api;
pub use self::batch_v1_api::{ BatchV1Api, BatchV1ApiClient };
mod batch_v1beta1_api;
pub use self::batch_v1beta1_api::{ BatchV1beta1Api, BatchV1beta1ApiClient };
mod certificates_api;
pub use self::certificates_api::{ CertificatesApi, CertificatesApiClient };
mod certificates_v1_api;
pub use self::certificates_v1_api::{ CertificatesV1Api, CertificatesV1ApiClient };
mod certificates_v1beta1_api;
pub use self::certificates_v1beta1_api::{ CertificatesV1beta1Api, CertificatesV1beta1ApiClient };
mod coordination_api;
pub use self::coordination_api::{ CoordinationApi, CoordinationApiClient };
mod coordination_v1_api;
pub use self::coordination_v1_api::{ CoordinationV1Api, CoordinationV1ApiClient };
mod coordination_v1beta1_api;
pub use self::coordination_v1beta1_api::{ CoordinationV1beta1Api, CoordinationV1beta1ApiClient };
mod core_api;
pub use self::core_api::{ CoreApi, CoreApiClient };
mod core_v1_api;
pub use self::core_v1_api::{ CoreV1Api, CoreV1ApiClient };
mod custom_objects_api;
pub use self::custom_objects_api::{ CustomObjectsApi, CustomObjectsApiClient };
mod discovery_api;
pub use self::discovery_api::{ DiscoveryApi, DiscoveryApiClient };
mod discovery_v1_api;
pub use self::discovery_v1_api::{ DiscoveryV1Api, DiscoveryV1ApiClient };
mod discovery_v1beta1_api;
pub use self::discovery_v1beta1_api::{ DiscoveryV1beta1Api, DiscoveryV1beta1ApiClient };
mod events_api;
pub use self::events_api::{ EventsApi, EventsApiClient };
mod events_v1_api;
pub use self::events_v1_api::{ EventsV1Api, EventsV1ApiClient };
mod events_v1beta1_api;
pub use self::events_v1beta1_api::{ EventsV1beta1Api, EventsV1beta1ApiClient };
mod extensions_api;
pub use self::extensions_api::{ ExtensionsApi, ExtensionsApiClient };
mod extensions_v1beta1_api;
pub use self::extensions_v1beta1_api::{ ExtensionsV1beta1Api, ExtensionsV1beta1ApiClient };
mod flowcontrol_apiserver_api;
pub use self::flowcontrol_apiserver_api::{ FlowcontrolApiserverApi, FlowcontrolApiserverApiClient };
mod flowcontrol_apiserver_v1beta1_api;
pub use self::flowcontrol_apiserver_v1beta1_api::{ FlowcontrolApiserverV1beta1Api, FlowcontrolApiserverV1beta1ApiClient };
mod internal_apiserver_api;
pub use self::internal_apiserver_api::{ InternalApiserverApi, InternalApiserverApiClient };
mod internal_apiserver_v1alpha1_api;
pub use self::internal_apiserver_v1alpha1_api::{ InternalApiserverV1alpha1Api, InternalApiserverV1alpha1ApiClient };
mod logs_api;
pub use self::logs_api::{ LogsApi, LogsApiClient };
mod networking_api;
pub use self::networking_api::{ NetworkingApi, NetworkingApiClient };
mod networking_v1_api;
pub use self::networking_v1_api::{ NetworkingV1Api, NetworkingV1ApiClient };
mod networking_v1beta1_api;
pub use self::networking_v1beta1_api::{ NetworkingV1beta1Api, NetworkingV1beta1ApiClient };
mod node_api;
pub use self::node_api::{ NodeApi, NodeApiClient };
mod node_v1_api;
pub use self::node_v1_api::{ NodeV1Api, NodeV1ApiClient };
mod node_v1alpha1_api;
pub use self::node_v1alpha1_api::{ NodeV1alpha1Api, NodeV1alpha1ApiClient };
mod node_v1beta1_api;
pub use self::node_v1beta1_api::{ NodeV1beta1Api, NodeV1beta1ApiClient };
mod openid_api;
pub use self::openid_api::{ OpenidApi, OpenidApiClient };
mod policy_api;
pub use self::policy_api::{ PolicyApi, PolicyApiClient };
mod policy_v1_api;
pub use self::policy_v1_api::{ PolicyV1Api, PolicyV1ApiClient };
mod policy_v1beta1_api;
pub use self::policy_v1beta1_api::{ PolicyV1beta1Api, PolicyV1beta1ApiClient };
mod rbac_authorization_api;
pub use self::rbac_authorization_api::{ RbacAuthorizationApi, RbacAuthorizationApiClient };
mod rbac_authorization_v1_api;
pub use self::rbac_authorization_v1_api::{ RbacAuthorizationV1Api, RbacAuthorizationV1ApiClient };
mod rbac_authorization_v1alpha1_api;
pub use self::rbac_authorization_v1alpha1_api::{ RbacAuthorizationV1alpha1Api, RbacAuthorizationV1alpha1ApiClient };
mod rbac_authorization_v1beta1_api;
pub use self::rbac_authorization_v1beta1_api::{ RbacAuthorizationV1beta1Api, RbacAuthorizationV1beta1ApiClient };
mod scheduling_api;
pub use self::scheduling_api::{ SchedulingApi, SchedulingApiClient };
mod scheduling_v1_api;
pub use self::scheduling_v1_api::{ SchedulingV1Api, SchedulingV1ApiClient };
mod scheduling_v1alpha1_api;
pub use self::scheduling_v1alpha1_api::{ SchedulingV1alpha1Api, SchedulingV1alpha1ApiClient };
mod scheduling_v1beta1_api;
pub use self::scheduling_v1beta1_api::{ SchedulingV1beta1Api, SchedulingV1beta1ApiClient };
mod storage_api;
pub use self::storage_api::{ StorageApi, StorageApiClient };
mod storage_v1_api;
pub use self::storage_v1_api::{ StorageV1Api, StorageV1ApiClient };
mod storage_v1alpha1_api;
pub use self::storage_v1alpha1_api::{ StorageV1alpha1Api, StorageV1alpha1ApiClient };
mod storage_v1beta1_api;
pub use self::storage_v1beta1_api::{ StorageV1beta1Api, StorageV1beta1ApiClient };
mod version_api;
pub use self::version_api::{ VersionApi, VersionApiClient };
mod well_known_api;
pub use self::well_known_api::{ WellKnownApi, WellKnownApiClient };

pub mod configuration;
pub mod client;
