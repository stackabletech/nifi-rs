# VersionedLabel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**label** | Option<**String**> | The text that appears in the label. | [optional]
**z_index** | Option<**i64**> | The z index of the connection. | [optional]
**width** | Option<**f64**> | The width of the label in pixels when at a 1:1 scale. | [optional]
**height** | Option<**f64**> | The height of the label in pixels when at a 1:1 scale. | [optional]
**style** | Option<**::std::collections::HashMap<String, String>**> | The styles for this label (font-size : 12px, background-color : #eee, etc). | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


