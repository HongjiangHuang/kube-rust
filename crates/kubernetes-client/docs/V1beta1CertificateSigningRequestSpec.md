# V1beta1CertificateSigningRequestSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> | Extra information about the requesting user. See user.Info interface for details. | [optional]
**groups** | Option<**Vec<String>**> | Group information about the requesting user. See user.Info interface for details. | [optional]
**request** | **String** | Base64-encoded PKCS#10 CSR data | 
**signer_name** | Option<**String**> | Requested signer for the request. It is a qualified name in the form: `scope-hostname.io/name`. If empty, it will be defaulted:  1. If it's a kubelet client certificate, it is assigned     \"kubernetes.io/kube-apiserver-client-kubelet\".  2. If it's a kubelet serving certificate, it is assigned     \"kubernetes.io/kubelet-serving\".  3. Otherwise, it is assigned \"kubernetes.io/legacy-unknown\". Distribution of trust for signers happens out of band. You can select on this field using `spec.signerName`. | [optional]
**uid** | Option<**String**> | UID information about the requesting user. See user.Info interface for details. | [optional]
**usages** | Option<**Vec<String>**> | allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3      https://tools.ietf.org/html/rfc5280#section-4.2.1.12 Valid values are:  \"signing\",  \"digital signature\",  \"content commitment\",  \"key encipherment\",  \"key agreement\",  \"data encipherment\",  \"cert sign\",  \"crl sign\",  \"encipher only\",  \"decipher only\",  \"any\",  \"server auth\",  \"client auth\",  \"code signing\",  \"email protection\",  \"s/mime\",  \"ipsec end system\",  \"ipsec tunnel\",  \"ipsec user\",  \"timestamping\",  \"ocsp signing\",  \"microsoft sgc\",  \"netscape sgc\" | [optional]
**username** | Option<**String**> | Information about the requesting user. See user.Info interface for details. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


