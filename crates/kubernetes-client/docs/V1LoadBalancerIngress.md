# V1LoadBalancerIngress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hostname** | Option<**String**> | Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers) | [optional]
**ip** | Option<**String**> | IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers) | [optional]
**ports** | Option<[**Vec<crate::models::V1PortStatus>**](v1.PortStatus.md)> | Ports is a list of records of service ports If used, every port defined in the service should have an entry in it | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


