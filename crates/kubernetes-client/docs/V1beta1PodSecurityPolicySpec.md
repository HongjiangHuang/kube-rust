# V1beta1PodSecurityPolicySpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_privilege_escalation** | Option<**bool**> | allowPrivilegeEscalation determines if a pod can request to allow privilege escalation. If unspecified, defaults to true. | [optional]
**allowed_csi_drivers** | Option<[**Vec<crate::models::V1beta1AllowedCsiDriver>**](v1beta1.AllowedCSIDriver.md)> | AllowedCSIDrivers is an allowlist of inline CSI drivers that must be explicitly set to be embedded within a pod spec. An empty value indicates that any CSI driver can be used for inline ephemeral volumes. This is a beta field, and is only honored if the API server enables the CSIInlineVolume feature gate. | [optional]
**allowed_capabilities** | Option<**Vec<String>**> | allowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author's discretion. You must not list a capability in both allowedCapabilities and requiredDropCapabilities. | [optional]
**allowed_flex_volumes** | Option<[**Vec<crate::models::V1beta1AllowedFlexVolume>**](v1beta1.AllowedFlexVolume.md)> | allowedFlexVolumes is an allowlist of Flexvolumes.  Empty or nil indicates that all Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes is allowed in the \"volumes\" field. | [optional]
**allowed_host_paths** | Option<[**Vec<crate::models::V1beta1AllowedHostPath>**](v1beta1.AllowedHostPath.md)> | allowedHostPaths is an allowlist of host paths. Empty indicates that all host paths may be used. | [optional]
**allowed_proc_mount_types** | Option<**Vec<String>**> | AllowedProcMountTypes is an allowlist of allowed ProcMountTypes. Empty or nil indicates that only the DefaultProcMountType may be used. This requires the ProcMountType feature flag to be enabled. | [optional]
**allowed_unsafe_sysctls** | Option<**Vec<String>**> | allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed. Kubelet has to allowlist all allowed unsafe sysctls explicitly to avoid rejection.  Examples: e.g. \"foo/_*\" allows \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" allows \"foo.bar\", \"foo.baz\", etc. | [optional]
**default_add_capabilities** | Option<**Vec<String>**> | defaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capability in both defaultAddCapabilities and requiredDropCapabilities. Capabilities added here are implicitly allowed, and need not be included in the allowedCapabilities list. | [optional]
**default_allow_privilege_escalation** | Option<**bool**> | defaultAllowPrivilegeEscalation controls the default setting for whether a process can gain more privileges than its parent process. | [optional]
**forbidden_sysctls** | Option<**Vec<String>**> | forbiddenSysctls is a list of explicitly forbidden sysctls, defaults to none. Each entry is either a plain sysctl name or ends in \"*\" in which case it is considered as a prefix of forbidden sysctls. Single * means all sysctls are forbidden.  Examples: e.g. \"foo/_*\" forbids \"foo/bar\", \"foo/baz\", etc. e.g. \"foo.*\" forbids \"foo.bar\", \"foo.baz\", etc. | [optional]
**fs_group** | [**crate::models::V1beta1FsGroupStrategyOptions**](v1beta1.FSGroupStrategyOptions.md) |  | 
**host_ipc** | Option<**bool**> | hostIPC determines if the policy allows the use of HostIPC in the pod spec. | [optional]
**host_network** | Option<**bool**> | hostNetwork determines if the policy allows the use of HostNetwork in the pod spec. | [optional]
**host_pid** | Option<**bool**> | hostPID determines if the policy allows the use of HostPID in the pod spec. | [optional]
**host_ports** | Option<[**Vec<crate::models::V1beta1HostPortRange>**](v1beta1.HostPortRange.md)> | hostPorts determines which host port ranges are allowed to be exposed. | [optional]
**privileged** | Option<**bool**> | privileged determines if a pod can request to be run as privileged. | [optional]
**read_only_root_filesystem** | Option<**bool**> | readOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to. | [optional]
**required_drop_capabilities** | Option<**Vec<String>**> | requiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added. | [optional]
**run_as_group** | Option<[**crate::models::V1beta1RunAsGroupStrategyOptions**](v1beta1.RunAsGroupStrategyOptions.md)> |  | [optional]
**run_as_user** | [**crate::models::V1beta1RunAsUserStrategyOptions**](v1beta1.RunAsUserStrategyOptions.md) |  | 
**runtime_class** | Option<[**crate::models::V1beta1RuntimeClassStrategyOptions**](v1beta1.RuntimeClassStrategyOptions.md)> |  | [optional]
**se_linux** | [**crate::models::V1beta1SeLinuxStrategyOptions**](v1beta1.SELinuxStrategyOptions.md) |  | 
**supplemental_groups** | [**crate::models::V1beta1SupplementalGroupsStrategyOptions**](v1beta1.SupplementalGroupsStrategyOptions.md) |  | 
**volumes** | Option<**Vec<String>**> | volumes is an allowlist of volume plugins. Empty indicates that no volumes may be used. To allow all volumes you may use '*'. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


