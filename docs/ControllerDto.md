# ControllerDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the NiFi. | [optional]
**name** | Option<**String**> | The name of the NiFi. | [optional]
**comments** | Option<**String**> | The comments for the NiFi. | [optional]
**running_count** | Option<**i32**> | The number of running components in the NiFi. | [optional]
**stopped_count** | Option<**i32**> | The number of stopped components in the NiFi. | [optional]
**invalid_count** | Option<**i32**> | The number of invalid components in the NiFi. | [optional]
**disabled_count** | Option<**i32**> | The number of disabled components in the NiFi. | [optional]
**active_remote_port_count** | Option<**i32**> | The number of active remote ports contained in the NiFi. | [optional]
**inactive_remote_port_count** | Option<**i32**> | The number of inactive remote ports contained in the NiFi. | [optional]
**input_port_count** | Option<**i32**> | The number of input ports contained in the NiFi. | [optional]
**output_port_count** | Option<**i32**> | The number of output ports in the NiFi. | [optional]
**remote_site_listening_port** | Option<**i32**> | The Socket Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null. | [optional]
**remote_site_http_listening_port** | Option<**i32**> | The HTTP(S) Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null. | [optional]
**site_to_site_secure** | Option<**bool**> | Indicates whether or not Site-to-Site communications with this instance is secure (2-way authentication). | [optional]
**instance_id** | Option<**String**> | If clustered, the id of the Cluster Manager, otherwise the id of the NiFi. | [optional]
**input_ports** | Option<[**Vec<crate::models::PortDto>**](PortDTO.md)> | The input ports available to send data to for the NiFi. | [optional]
**output_ports** | Option<[**Vec<crate::models::PortDto>**](PortDTO.md)> | The output ports available to received data from the NiFi. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


