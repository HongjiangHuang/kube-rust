# ExtensionsV1beta1IngressSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backend** | Option<[**crate::models::ExtensionsV1beta1IngressBackend**](extensions.v1beta1.IngressBackend.md)> |  | [optional]
**ingress_class_name** | Option<**String**> | IngressClassName is the name of the IngressClass cluster resource. The associated IngressClass defines which controller will implement the resource. This replaces the deprecated `kubernetes.io/ingress.class` annotation. For backwards compatibility, when that annotation is set, it must be given precedence over this field. The controller may emit a warning if the field and annotation have different values. Implementations of this API should ignore Ingresses without a class specified. An IngressClass resource may be marked as default, which can be used to set a default value for this field. For more information, refer to the IngressClass documentation. | [optional]
**rules** | Option<[**Vec<crate::models::ExtensionsV1beta1IngressRule>**](extensions.v1beta1.IngressRule.md)> | A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend. | [optional]
**tls** | Option<[**Vec<crate::models::ExtensionsV1beta1IngressTls>**](extensions.v1beta1.IngressTLS.md)> | TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


