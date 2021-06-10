use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    admissionregistration_api: Box<dyn crate::apis::AdmissionregistrationApi>,
    admissionregistration_v1_api: Box<dyn crate::apis::AdmissionregistrationV1Api>,
    admissionregistration_v1beta1_api: Box<dyn crate::apis::AdmissionregistrationV1beta1Api>,
    apiextensions_api: Box<dyn crate::apis::ApiextensionsApi>,
    apiextensions_v1_api: Box<dyn crate::apis::ApiextensionsV1Api>,
    apiextensions_v1beta1_api: Box<dyn crate::apis::ApiextensionsV1beta1Api>,
    apiregistration_api: Box<dyn crate::apis::ApiregistrationApi>,
    apiregistration_v1_api: Box<dyn crate::apis::ApiregistrationV1Api>,
    apiregistration_v1beta1_api: Box<dyn crate::apis::ApiregistrationV1beta1Api>,
    apis_api: Box<dyn crate::apis::ApisApi>,
    apps_api: Box<dyn crate::apis::AppsApi>,
    apps_v1_api: Box<dyn crate::apis::AppsV1Api>,
    authentication_api: Box<dyn crate::apis::AuthenticationApi>,
    authentication_v1_api: Box<dyn crate::apis::AuthenticationV1Api>,
    authentication_v1beta1_api: Box<dyn crate::apis::AuthenticationV1beta1Api>,
    authorization_api: Box<dyn crate::apis::AuthorizationApi>,
    authorization_v1_api: Box<dyn crate::apis::AuthorizationV1Api>,
    authorization_v1beta1_api: Box<dyn crate::apis::AuthorizationV1beta1Api>,
    autoscaling_api: Box<dyn crate::apis::AutoscalingApi>,
    autoscaling_v1_api: Box<dyn crate::apis::AutoscalingV1Api>,
    autoscaling_v2beta1_api: Box<dyn crate::apis::AutoscalingV2beta1Api>,
    autoscaling_v2beta2_api: Box<dyn crate::apis::AutoscalingV2beta2Api>,
    batch_api: Box<dyn crate::apis::BatchApi>,
    batch_v1_api: Box<dyn crate::apis::BatchV1Api>,
    batch_v1beta1_api: Box<dyn crate::apis::BatchV1beta1Api>,
    certificates_api: Box<dyn crate::apis::CertificatesApi>,
    certificates_v1_api: Box<dyn crate::apis::CertificatesV1Api>,
    certificates_v1beta1_api: Box<dyn crate::apis::CertificatesV1beta1Api>,
    coordination_api: Box<dyn crate::apis::CoordinationApi>,
    coordination_v1_api: Box<dyn crate::apis::CoordinationV1Api>,
    coordination_v1beta1_api: Box<dyn crate::apis::CoordinationV1beta1Api>,
    core_api: Box<dyn crate::apis::CoreApi>,
    core_v1_api: Box<dyn crate::apis::CoreV1Api>,
    custom_objects_api: Box<dyn crate::apis::CustomObjectsApi>,
    discovery_api: Box<dyn crate::apis::DiscoveryApi>,
    discovery_v1_api: Box<dyn crate::apis::DiscoveryV1Api>,
    discovery_v1beta1_api: Box<dyn crate::apis::DiscoveryV1beta1Api>,
    events_api: Box<dyn crate::apis::EventsApi>,
    events_v1_api: Box<dyn crate::apis::EventsV1Api>,
    events_v1beta1_api: Box<dyn crate::apis::EventsV1beta1Api>,
    extensions_api: Box<dyn crate::apis::ExtensionsApi>,
    extensions_v1beta1_api: Box<dyn crate::apis::ExtensionsV1beta1Api>,
    flowcontrol_apiserver_api: Box<dyn crate::apis::FlowcontrolApiserverApi>,
    flowcontrol_apiserver_v1beta1_api: Box<dyn crate::apis::FlowcontrolApiserverV1beta1Api>,
    internal_apiserver_api: Box<dyn crate::apis::InternalApiserverApi>,
    internal_apiserver_v1alpha1_api: Box<dyn crate::apis::InternalApiserverV1alpha1Api>,
    logs_api: Box<dyn crate::apis::LogsApi>,
    networking_api: Box<dyn crate::apis::NetworkingApi>,
    networking_v1_api: Box<dyn crate::apis::NetworkingV1Api>,
    networking_v1beta1_api: Box<dyn crate::apis::NetworkingV1beta1Api>,
    node_api: Box<dyn crate::apis::NodeApi>,
    node_v1_api: Box<dyn crate::apis::NodeV1Api>,
    node_v1alpha1_api: Box<dyn crate::apis::NodeV1alpha1Api>,
    node_v1beta1_api: Box<dyn crate::apis::NodeV1beta1Api>,
    openid_api: Box<dyn crate::apis::OpenidApi>,
    policy_api: Box<dyn crate::apis::PolicyApi>,
    policy_v1_api: Box<dyn crate::apis::PolicyV1Api>,
    policy_v1beta1_api: Box<dyn crate::apis::PolicyV1beta1Api>,
    rbac_authorization_api: Box<dyn crate::apis::RbacAuthorizationApi>,
    rbac_authorization_v1_api: Box<dyn crate::apis::RbacAuthorizationV1Api>,
    rbac_authorization_v1alpha1_api: Box<dyn crate::apis::RbacAuthorizationV1alpha1Api>,
    rbac_authorization_v1beta1_api: Box<dyn crate::apis::RbacAuthorizationV1beta1Api>,
    scheduling_api: Box<dyn crate::apis::SchedulingApi>,
    scheduling_v1_api: Box<dyn crate::apis::SchedulingV1Api>,
    scheduling_v1alpha1_api: Box<dyn crate::apis::SchedulingV1alpha1Api>,
    scheduling_v1beta1_api: Box<dyn crate::apis::SchedulingV1beta1Api>,
    storage_api: Box<dyn crate::apis::StorageApi>,
    storage_v1_api: Box<dyn crate::apis::StorageV1Api>,
    storage_v1alpha1_api: Box<dyn crate::apis::StorageV1alpha1Api>,
    storage_v1beta1_api: Box<dyn crate::apis::StorageV1beta1Api>,
    version_api: Box<dyn crate::apis::VersionApi>,
    well_known_api: Box<dyn crate::apis::WellKnownApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            admissionregistration_api: Box::new(crate::apis::AdmissionregistrationApiClient::new(rc.clone())),
            admissionregistration_v1_api: Box::new(crate::apis::AdmissionregistrationV1ApiClient::new(rc.clone())),
            admissionregistration_v1beta1_api: Box::new(crate::apis::AdmissionregistrationV1beta1ApiClient::new(rc.clone())),
            apiextensions_api: Box::new(crate::apis::ApiextensionsApiClient::new(rc.clone())),
            apiextensions_v1_api: Box::new(crate::apis::ApiextensionsV1ApiClient::new(rc.clone())),
            apiextensions_v1beta1_api: Box::new(crate::apis::ApiextensionsV1beta1ApiClient::new(rc.clone())),
            apiregistration_api: Box::new(crate::apis::ApiregistrationApiClient::new(rc.clone())),
            apiregistration_v1_api: Box::new(crate::apis::ApiregistrationV1ApiClient::new(rc.clone())),
            apiregistration_v1beta1_api: Box::new(crate::apis::ApiregistrationV1beta1ApiClient::new(rc.clone())),
            apis_api: Box::new(crate::apis::ApisApiClient::new(rc.clone())),
            apps_api: Box::new(crate::apis::AppsApiClient::new(rc.clone())),
            apps_v1_api: Box::new(crate::apis::AppsV1ApiClient::new(rc.clone())),
            authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(rc.clone())),
            authentication_v1_api: Box::new(crate::apis::AuthenticationV1ApiClient::new(rc.clone())),
            authentication_v1beta1_api: Box::new(crate::apis::AuthenticationV1beta1ApiClient::new(rc.clone())),
            authorization_api: Box::new(crate::apis::AuthorizationApiClient::new(rc.clone())),
            authorization_v1_api: Box::new(crate::apis::AuthorizationV1ApiClient::new(rc.clone())),
            authorization_v1beta1_api: Box::new(crate::apis::AuthorizationV1beta1ApiClient::new(rc.clone())),
            autoscaling_api: Box::new(crate::apis::AutoscalingApiClient::new(rc.clone())),
            autoscaling_v1_api: Box::new(crate::apis::AutoscalingV1ApiClient::new(rc.clone())),
            autoscaling_v2beta1_api: Box::new(crate::apis::AutoscalingV2beta1ApiClient::new(rc.clone())),
            autoscaling_v2beta2_api: Box::new(crate::apis::AutoscalingV2beta2ApiClient::new(rc.clone())),
            batch_api: Box::new(crate::apis::BatchApiClient::new(rc.clone())),
            batch_v1_api: Box::new(crate::apis::BatchV1ApiClient::new(rc.clone())),
            batch_v1beta1_api: Box::new(crate::apis::BatchV1beta1ApiClient::new(rc.clone())),
            certificates_api: Box::new(crate::apis::CertificatesApiClient::new(rc.clone())),
            certificates_v1_api: Box::new(crate::apis::CertificatesV1ApiClient::new(rc.clone())),
            certificates_v1beta1_api: Box::new(crate::apis::CertificatesV1beta1ApiClient::new(rc.clone())),
            coordination_api: Box::new(crate::apis::CoordinationApiClient::new(rc.clone())),
            coordination_v1_api: Box::new(crate::apis::CoordinationV1ApiClient::new(rc.clone())),
            coordination_v1beta1_api: Box::new(crate::apis::CoordinationV1beta1ApiClient::new(rc.clone())),
            core_api: Box::new(crate::apis::CoreApiClient::new(rc.clone())),
            core_v1_api: Box::new(crate::apis::CoreV1ApiClient::new(rc.clone())),
            custom_objects_api: Box::new(crate::apis::CustomObjectsApiClient::new(rc.clone())),
            discovery_api: Box::new(crate::apis::DiscoveryApiClient::new(rc.clone())),
            discovery_v1_api: Box::new(crate::apis::DiscoveryV1ApiClient::new(rc.clone())),
            discovery_v1beta1_api: Box::new(crate::apis::DiscoveryV1beta1ApiClient::new(rc.clone())),
            events_api: Box::new(crate::apis::EventsApiClient::new(rc.clone())),
            events_v1_api: Box::new(crate::apis::EventsV1ApiClient::new(rc.clone())),
            events_v1beta1_api: Box::new(crate::apis::EventsV1beta1ApiClient::new(rc.clone())),
            extensions_api: Box::new(crate::apis::ExtensionsApiClient::new(rc.clone())),
            extensions_v1beta1_api: Box::new(crate::apis::ExtensionsV1beta1ApiClient::new(rc.clone())),
            flowcontrol_apiserver_api: Box::new(crate::apis::FlowcontrolApiserverApiClient::new(rc.clone())),
            flowcontrol_apiserver_v1beta1_api: Box::new(crate::apis::FlowcontrolApiserverV1beta1ApiClient::new(rc.clone())),
            internal_apiserver_api: Box::new(crate::apis::InternalApiserverApiClient::new(rc.clone())),
            internal_apiserver_v1alpha1_api: Box::new(crate::apis::InternalApiserverV1alpha1ApiClient::new(rc.clone())),
            logs_api: Box::new(crate::apis::LogsApiClient::new(rc.clone())),
            networking_api: Box::new(crate::apis::NetworkingApiClient::new(rc.clone())),
            networking_v1_api: Box::new(crate::apis::NetworkingV1ApiClient::new(rc.clone())),
            networking_v1beta1_api: Box::new(crate::apis::NetworkingV1beta1ApiClient::new(rc.clone())),
            node_api: Box::new(crate::apis::NodeApiClient::new(rc.clone())),
            node_v1_api: Box::new(crate::apis::NodeV1ApiClient::new(rc.clone())),
            node_v1alpha1_api: Box::new(crate::apis::NodeV1alpha1ApiClient::new(rc.clone())),
            node_v1beta1_api: Box::new(crate::apis::NodeV1beta1ApiClient::new(rc.clone())),
            openid_api: Box::new(crate::apis::OpenidApiClient::new(rc.clone())),
            policy_api: Box::new(crate::apis::PolicyApiClient::new(rc.clone())),
            policy_v1_api: Box::new(crate::apis::PolicyV1ApiClient::new(rc.clone())),
            policy_v1beta1_api: Box::new(crate::apis::PolicyV1beta1ApiClient::new(rc.clone())),
            rbac_authorization_api: Box::new(crate::apis::RbacAuthorizationApiClient::new(rc.clone())),
            rbac_authorization_v1_api: Box::new(crate::apis::RbacAuthorizationV1ApiClient::new(rc.clone())),
            rbac_authorization_v1alpha1_api: Box::new(crate::apis::RbacAuthorizationV1alpha1ApiClient::new(rc.clone())),
            rbac_authorization_v1beta1_api: Box::new(crate::apis::RbacAuthorizationV1beta1ApiClient::new(rc.clone())),
            scheduling_api: Box::new(crate::apis::SchedulingApiClient::new(rc.clone())),
            scheduling_v1_api: Box::new(crate::apis::SchedulingV1ApiClient::new(rc.clone())),
            scheduling_v1alpha1_api: Box::new(crate::apis::SchedulingV1alpha1ApiClient::new(rc.clone())),
            scheduling_v1beta1_api: Box::new(crate::apis::SchedulingV1beta1ApiClient::new(rc.clone())),
            storage_api: Box::new(crate::apis::StorageApiClient::new(rc.clone())),
            storage_v1_api: Box::new(crate::apis::StorageV1ApiClient::new(rc.clone())),
            storage_v1alpha1_api: Box::new(crate::apis::StorageV1alpha1ApiClient::new(rc.clone())),
            storage_v1beta1_api: Box::new(crate::apis::StorageV1beta1ApiClient::new(rc.clone())),
            version_api: Box::new(crate::apis::VersionApiClient::new(rc.clone())),
            well_known_api: Box::new(crate::apis::WellKnownApiClient::new(rc.clone())),
        }
    }

    pub fn admissionregistration_api(&self) -> &dyn crate::apis::AdmissionregistrationApi{
        self.admissionregistration_api.as_ref()
    }

    pub fn admissionregistration_v1_api(&self) -> &dyn crate::apis::AdmissionregistrationV1Api{
        self.admissionregistration_v1_api.as_ref()
    }

    pub fn admissionregistration_v1beta1_api(&self) -> &dyn crate::apis::AdmissionregistrationV1beta1Api{
        self.admissionregistration_v1beta1_api.as_ref()
    }

    pub fn apiextensions_api(&self) -> &dyn crate::apis::ApiextensionsApi{
        self.apiextensions_api.as_ref()
    }

    pub fn apiextensions_v1_api(&self) -> &dyn crate::apis::ApiextensionsV1Api{
        self.apiextensions_v1_api.as_ref()
    }

    pub fn apiextensions_v1beta1_api(&self) -> &dyn crate::apis::ApiextensionsV1beta1Api{
        self.apiextensions_v1beta1_api.as_ref()
    }

    pub fn apiregistration_api(&self) -> &dyn crate::apis::ApiregistrationApi{
        self.apiregistration_api.as_ref()
    }

    pub fn apiregistration_v1_api(&self) -> &dyn crate::apis::ApiregistrationV1Api{
        self.apiregistration_v1_api.as_ref()
    }

    pub fn apiregistration_v1beta1_api(&self) -> &dyn crate::apis::ApiregistrationV1beta1Api{
        self.apiregistration_v1beta1_api.as_ref()
    }

    pub fn apis_api(&self) -> &dyn crate::apis::ApisApi{
        self.apis_api.as_ref()
    }

    pub fn apps_api(&self) -> &dyn crate::apis::AppsApi{
        self.apps_api.as_ref()
    }

    pub fn apps_v1_api(&self) -> &dyn crate::apis::AppsV1Api{
        self.apps_v1_api.as_ref()
    }

    pub fn authentication_api(&self) -> &dyn crate::apis::AuthenticationApi{
        self.authentication_api.as_ref()
    }

    pub fn authentication_v1_api(&self) -> &dyn crate::apis::AuthenticationV1Api{
        self.authentication_v1_api.as_ref()
    }

    pub fn authentication_v1beta1_api(&self) -> &dyn crate::apis::AuthenticationV1beta1Api{
        self.authentication_v1beta1_api.as_ref()
    }

    pub fn authorization_api(&self) -> &dyn crate::apis::AuthorizationApi{
        self.authorization_api.as_ref()
    }

    pub fn authorization_v1_api(&self) -> &dyn crate::apis::AuthorizationV1Api{
        self.authorization_v1_api.as_ref()
    }

    pub fn authorization_v1beta1_api(&self) -> &dyn crate::apis::AuthorizationV1beta1Api{
        self.authorization_v1beta1_api.as_ref()
    }

    pub fn autoscaling_api(&self) -> &dyn crate::apis::AutoscalingApi{
        self.autoscaling_api.as_ref()
    }

    pub fn autoscaling_v1_api(&self) -> &dyn crate::apis::AutoscalingV1Api{
        self.autoscaling_v1_api.as_ref()
    }

    pub fn autoscaling_v2beta1_api(&self) -> &dyn crate::apis::AutoscalingV2beta1Api{
        self.autoscaling_v2beta1_api.as_ref()
    }

    pub fn autoscaling_v2beta2_api(&self) -> &dyn crate::apis::AutoscalingV2beta2Api{
        self.autoscaling_v2beta2_api.as_ref()
    }

    pub fn batch_api(&self) -> &dyn crate::apis::BatchApi{
        self.batch_api.as_ref()
    }

    pub fn batch_v1_api(&self) -> &dyn crate::apis::BatchV1Api{
        self.batch_v1_api.as_ref()
    }

    pub fn batch_v1beta1_api(&self) -> &dyn crate::apis::BatchV1beta1Api{
        self.batch_v1beta1_api.as_ref()
    }

    pub fn certificates_api(&self) -> &dyn crate::apis::CertificatesApi{
        self.certificates_api.as_ref()
    }

    pub fn certificates_v1_api(&self) -> &dyn crate::apis::CertificatesV1Api{
        self.certificates_v1_api.as_ref()
    }

    pub fn certificates_v1beta1_api(&self) -> &dyn crate::apis::CertificatesV1beta1Api{
        self.certificates_v1beta1_api.as_ref()
    }

    pub fn coordination_api(&self) -> &dyn crate::apis::CoordinationApi{
        self.coordination_api.as_ref()
    }

    pub fn coordination_v1_api(&self) -> &dyn crate::apis::CoordinationV1Api{
        self.coordination_v1_api.as_ref()
    }

    pub fn coordination_v1beta1_api(&self) -> &dyn crate::apis::CoordinationV1beta1Api{
        self.coordination_v1beta1_api.as_ref()
    }

    pub fn core_api(&self) -> &dyn crate::apis::CoreApi{
        self.core_api.as_ref()
    }

    pub fn core_v1_api(&self) -> &dyn crate::apis::CoreV1Api{
        self.core_v1_api.as_ref()
    }

    pub fn custom_objects_api(&self) -> &dyn crate::apis::CustomObjectsApi{
        self.custom_objects_api.as_ref()
    }

    pub fn discovery_api(&self) -> &dyn crate::apis::DiscoveryApi{
        self.discovery_api.as_ref()
    }

    pub fn discovery_v1_api(&self) -> &dyn crate::apis::DiscoveryV1Api{
        self.discovery_v1_api.as_ref()
    }

    pub fn discovery_v1beta1_api(&self) -> &dyn crate::apis::DiscoveryV1beta1Api{
        self.discovery_v1beta1_api.as_ref()
    }

    pub fn events_api(&self) -> &dyn crate::apis::EventsApi{
        self.events_api.as_ref()
    }

    pub fn events_v1_api(&self) -> &dyn crate::apis::EventsV1Api{
        self.events_v1_api.as_ref()
    }

    pub fn events_v1beta1_api(&self) -> &dyn crate::apis::EventsV1beta1Api{
        self.events_v1beta1_api.as_ref()
    }

    pub fn extensions_api(&self) -> &dyn crate::apis::ExtensionsApi{
        self.extensions_api.as_ref()
    }

    pub fn extensions_v1beta1_api(&self) -> &dyn crate::apis::ExtensionsV1beta1Api{
        self.extensions_v1beta1_api.as_ref()
    }

    pub fn flowcontrol_apiserver_api(&self) -> &dyn crate::apis::FlowcontrolApiserverApi{
        self.flowcontrol_apiserver_api.as_ref()
    }

    pub fn flowcontrol_apiserver_v1beta1_api(&self) -> &dyn crate::apis::FlowcontrolApiserverV1beta1Api{
        self.flowcontrol_apiserver_v1beta1_api.as_ref()
    }

    pub fn internal_apiserver_api(&self) -> &dyn crate::apis::InternalApiserverApi{
        self.internal_apiserver_api.as_ref()
    }

    pub fn internal_apiserver_v1alpha1_api(&self) -> &dyn crate::apis::InternalApiserverV1alpha1Api{
        self.internal_apiserver_v1alpha1_api.as_ref()
    }

    pub fn logs_api(&self) -> &dyn crate::apis::LogsApi{
        self.logs_api.as_ref()
    }

    pub fn networking_api(&self) -> &dyn crate::apis::NetworkingApi{
        self.networking_api.as_ref()
    }

    pub fn networking_v1_api(&self) -> &dyn crate::apis::NetworkingV1Api{
        self.networking_v1_api.as_ref()
    }

    pub fn networking_v1beta1_api(&self) -> &dyn crate::apis::NetworkingV1beta1Api{
        self.networking_v1beta1_api.as_ref()
    }

    pub fn node_api(&self) -> &dyn crate::apis::NodeApi{
        self.node_api.as_ref()
    }

    pub fn node_v1_api(&self) -> &dyn crate::apis::NodeV1Api{
        self.node_v1_api.as_ref()
    }

    pub fn node_v1alpha1_api(&self) -> &dyn crate::apis::NodeV1alpha1Api{
        self.node_v1alpha1_api.as_ref()
    }

    pub fn node_v1beta1_api(&self) -> &dyn crate::apis::NodeV1beta1Api{
        self.node_v1beta1_api.as_ref()
    }

    pub fn openid_api(&self) -> &dyn crate::apis::OpenidApi{
        self.openid_api.as_ref()
    }

    pub fn policy_api(&self) -> &dyn crate::apis::PolicyApi{
        self.policy_api.as_ref()
    }

    pub fn policy_v1_api(&self) -> &dyn crate::apis::PolicyV1Api{
        self.policy_v1_api.as_ref()
    }

    pub fn policy_v1beta1_api(&self) -> &dyn crate::apis::PolicyV1beta1Api{
        self.policy_v1beta1_api.as_ref()
    }

    pub fn rbac_authorization_api(&self) -> &dyn crate::apis::RbacAuthorizationApi{
        self.rbac_authorization_api.as_ref()
    }

    pub fn rbac_authorization_v1_api(&self) -> &dyn crate::apis::RbacAuthorizationV1Api{
        self.rbac_authorization_v1_api.as_ref()
    }

    pub fn rbac_authorization_v1alpha1_api(&self) -> &dyn crate::apis::RbacAuthorizationV1alpha1Api{
        self.rbac_authorization_v1alpha1_api.as_ref()
    }

    pub fn rbac_authorization_v1beta1_api(&self) -> &dyn crate::apis::RbacAuthorizationV1beta1Api{
        self.rbac_authorization_v1beta1_api.as_ref()
    }

    pub fn scheduling_api(&self) -> &dyn crate::apis::SchedulingApi{
        self.scheduling_api.as_ref()
    }

    pub fn scheduling_v1_api(&self) -> &dyn crate::apis::SchedulingV1Api{
        self.scheduling_v1_api.as_ref()
    }

    pub fn scheduling_v1alpha1_api(&self) -> &dyn crate::apis::SchedulingV1alpha1Api{
        self.scheduling_v1alpha1_api.as_ref()
    }

    pub fn scheduling_v1beta1_api(&self) -> &dyn crate::apis::SchedulingV1beta1Api{
        self.scheduling_v1beta1_api.as_ref()
    }

    pub fn storage_api(&self) -> &dyn crate::apis::StorageApi{
        self.storage_api.as_ref()
    }

    pub fn storage_v1_api(&self) -> &dyn crate::apis::StorageV1Api{
        self.storage_v1_api.as_ref()
    }

    pub fn storage_v1alpha1_api(&self) -> &dyn crate::apis::StorageV1alpha1Api{
        self.storage_v1alpha1_api.as_ref()
    }

    pub fn storage_v1beta1_api(&self) -> &dyn crate::apis::StorageV1beta1Api{
        self.storage_v1beta1_api.as_ref()
    }

    pub fn version_api(&self) -> &dyn crate::apis::VersionApi{
        self.version_api.as_ref()
    }

    pub fn well_known_api(&self) -> &dyn crate::apis::WellKnownApi{
        self.well_known_api.as_ref()
    }

}
