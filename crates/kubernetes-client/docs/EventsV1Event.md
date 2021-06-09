# EventsV1Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | action is what action was taken/failed regarding to the regarding object. It is machine-readable. This field cannot be empty for new Events and it can have at most 128 characters. | [optional]
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**deprecated_count** | Option<**i32**> | deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type. | [optional]
**deprecated_first_timestamp** | Option<**String**> | deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type. | [optional]
**deprecated_last_timestamp** | Option<**String**> | deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type. | [optional]
**deprecated_source** | Option<[**crate::models::V1EventSource**](v1.EventSource.md)> |  | [optional]
**event_time** | **String** | eventTime is the time when this Event was first observed. It is required. | 
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | Option<[**crate::models::V1ObjectMeta**](v1.ObjectMeta.md)> |  | [optional]
**note** | Option<**String**> | note is a human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB. | [optional]
**reason** | Option<**String**> | reason is why the action was taken. It is human-readable. This field cannot be empty for new Events and it can have at most 128 characters. | [optional]
**regarding** | Option<[**crate::models::V1ObjectReference**](v1.ObjectReference.md)> |  | [optional]
**related** | Option<[**crate::models::V1ObjectReference**](v1.ObjectReference.md)> |  | [optional]
**reporting_controller** | Option<**String**> | reportingController is the name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. This field cannot be empty for new Events. | [optional]
**reporting_instance** | Option<**String**> | reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`. This field cannot be empty for new Events and it can have at most 128 characters. | [optional]
**series** | Option<[**crate::models::EventsV1EventSeries**](events.v1.EventSeries.md)> |  | [optional]
**_type** | Option<**String**> | type is the type of this event (Normal, Warning), new types could be added in the future. It is machine-readable. This field cannot be empty for new Events. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


