# SnippetDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the snippet. | [optional]
**uri** | Option<**String**> | The URI of the snippet. | [optional]
**parent_group_id** | Option<**String**> | The group id for the components in the snippet. | [optional]
**process_groups** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the process groups in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**remote_process_groups** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the remote process groups in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**processors** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the processors in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**input_ports** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the input ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**output_ports** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the output ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**connections** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the connections in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**labels** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the labels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]
**funnels** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | The ids of the funnels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


