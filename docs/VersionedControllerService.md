# VersionedControllerService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**_type** | Option<**String**> | The type of the extension component | [optional]
**bundle** | Option<[**crate::models::Bundle**](Bundle.md)> |  | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties for the component. Properties whose value is not set will only contain the property name. | [optional]
**property_descriptors** | Option<[**::std::collections::HashMap<String, crate::models::VersionedPropertyDescriptor>**](VersionedPropertyDescriptor.md)> | The property descriptors for the component. | [optional]
**controller_service_apis** | Option<[**Vec<crate::models::ControllerServiceApi>**](ControllerServiceAPI.md)> | Lists the APIs this Controller Service implements. | [optional]
**annotation_data** | Option<**String**> | The annotation for the controller service. This is how the custom UI relays configuration to the controller service. | [optional]
**scheduled_state** | Option<**String**> | The ScheduledState denoting whether the Controller Service is ENABLED or DISABLED | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


