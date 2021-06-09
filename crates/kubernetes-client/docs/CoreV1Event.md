# CoreV1Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | What action was taken/failed regarding to the Regarding object. | [optional]
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**count** | Option<**i32**> | The number of times this event has occurred. | [optional]
**event_time** | Option<**String**> | Time when this Event was first observed. | [optional]
**first_timestamp** | Option<**String**> | The time at which the event was first recorded. (Time of server receipt is in TypeMeta.) | [optional]
**involved_object** | [**crate::models::V1ObjectReference**](v1.ObjectReference.md) |  | 
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**last_timestamp** | Option<**String**> | The time at which the most recent occurrence of this event was recorded. | [optional]
**message** | Option<**String**> | A human-readable description of the status of this operation. | [optional]
**metadata** | [**crate::models::V1ObjectMeta**](v1.ObjectMeta.md) |  | 
**reason** | Option<**String**> | This should be a short, machine understandable string that gives the reason for the transition into the object's current status. | [optional]
**related** | Option<[**crate::models::V1ObjectReference**](v1.ObjectReference.md)> |  | [optional]
**reporting_component** | Option<**String**> | Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. | [optional]
**reporting_instance** | Option<**String**> | ID of the controller instance, e.g. `kubelet-xyzf`. | [optional]
**series** | Option<[**crate::models::CoreV1EventSeries**](core.v1.EventSeries.md)> |  | [optional]
**source** | Option<[**crate::models::V1EventSource**](v1.EventSource.md)> |  | [optional]
**_type** | Option<**String**> | Type of this event (Normal, Warning), new types could be added in the future | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


