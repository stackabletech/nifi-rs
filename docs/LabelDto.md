# LabelDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**label** | Option<**String**> | The text that appears in the label. | [optional]
**width** | Option<**f64**> | The width of the label in pixels when at a 1:1 scale. | [optional]
**height** | Option<**f64**> | The height of the label in pixels when at a 1:1 scale. | [optional]
**getz_index** | Option<**i64**> | The z index of the label. | [optional]
**style** | Option<**::std::collections::HashMap<String, String>**> | The styles for this label (font-size : 12px, background-color : #eee, etc). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


