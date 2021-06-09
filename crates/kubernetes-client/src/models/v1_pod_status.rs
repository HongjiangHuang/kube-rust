/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PodStatus : PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1PodStatus {
    /// Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1PodCondition>>,
    /// The list has one entry per container in the manifest. Each entry is currently the output of `docker inspect`. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    #[serde(rename = "containerStatuses", skip_serializing_if = "Option::is_none")]
    pub container_statuses: Option<Vec<crate::models::V1ContainerStatus>>,
    /// Status for any ephemeral containers that have run in this pod. This field is alpha-level and is only populated by servers that enable the EphemeralContainers feature.
    #[serde(rename = "ephemeralContainerStatuses", skip_serializing_if = "Option::is_none")]
    pub ephemeral_container_statuses: Option<Vec<crate::models::V1ContainerStatus>>,
    /// IP address of the host to which the pod is assigned. Empty if not yet scheduled.
    #[serde(rename = "hostIP", skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    #[serde(rename = "initContainerStatuses", skip_serializing_if = "Option::is_none")]
    pub init_container_statuses: Option<Vec<crate::models::V1ContainerStatus>>,
    /// A human readable message indicating details about why the pod is in this condition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.
    #[serde(rename = "nominatedNodeName", skip_serializing_if = "Option::is_none")]
    pub nominated_node_name: Option<String>,
    /// The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:  Pending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.  More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// IP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
    #[serde(rename = "podIP", skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,
    /// podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.
    #[serde(rename = "podIPs", skip_serializing_if = "Option::is_none")]
    pub pod_i_ps: Option<Vec<crate::models::V1PodIp>>,
    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://git.k8s.io/community/contributors/design-proposals/node/resource-qos.md
    #[serde(rename = "qosClass", skip_serializing_if = "Option::is_none")]
    pub qos_class: Option<String>,
    /// A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl V1PodStatus {
    /// PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.
    pub fn new() -> V1PodStatus {
        V1PodStatus {
            conditions: None,
            container_statuses: None,
            ephemeral_container_statuses: None,
            host_ip: None,
            init_container_statuses: None,
            message: None,
            nominated_node_name: None,
            phase: None,
            pod_ip: None,
            pod_i_ps: None,
            qos_class: None,
            reason: None,
            start_time: None,
        }
    }
}

