/*
 * NiFi Rest API
 *
 * The Rest API provides programmatic access to command and control a NiFi instance in real time. Start and                                             stop processors, monitor queues, query provenance data, and more. Each endpoint below includes a description,                                             definitions of the expected input and output, potential response codes, and the authorizations required                                             to invoke each service.
 *
 * The version of the OpenAPI document: 1.16.0
 * Contact: dev@nifi.apache.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectionStatusPredictionsSnapshotDto {
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    #[serde(rename = "predictedMillisUntilCountBackpressure", skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    #[serde(rename = "predictedMillisUntilBytesBackpressure", skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The configured interval (in seconds) for predicting connection queue count and size (and percent usage).
    #[serde(rename = "predictionIntervalSeconds", skip_serializing_if = "Option::is_none")]
    pub prediction_interval_seconds: Option<i32>,
    /// The predicted number of queued objects at the next configured interval.
    #[serde(rename = "predictedCountAtNextInterval", skip_serializing_if = "Option::is_none")]
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted total number of bytes in the queue at the next configured interval.
    #[serde(rename = "predictedBytesAtNextInterval", skip_serializing_if = "Option::is_none")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// Predicted connection percent use regarding queued flow files count and backpressure threshold if configured.
    #[serde(rename = "predictedPercentCount", skip_serializing_if = "Option::is_none")]
    pub predicted_percent_count: Option<i32>,
    /// Predicted connection percent use regarding queued flow files size and backpressure threshold if configured.
    #[serde(rename = "predictedPercentBytes", skip_serializing_if = "Option::is_none")]
    pub predicted_percent_bytes: Option<i32>,
}

impl ConnectionStatusPredictionsSnapshotDto {
    pub fn new() -> ConnectionStatusPredictionsSnapshotDto {
        ConnectionStatusPredictionsSnapshotDto {
            predicted_millis_until_count_backpressure: None,
            predicted_millis_until_bytes_backpressure: None,
            prediction_interval_seconds: None,
            predicted_count_at_next_interval: None,
            predicted_bytes_at_next_interval: None,
            predicted_percent_count: None,
            predicted_percent_bytes: None,
        }
    }
}


