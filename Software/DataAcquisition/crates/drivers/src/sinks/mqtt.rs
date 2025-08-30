#![cfg(feature = "std")]

use super::{SensorSample, Sink};
use rumqttc::{Client, MqttOptions, QoS};
use serde::Serialize;
use std::fmt;
use std::thread;
use std::time::Duration;



/* ---------- error type --------------------------------------- */

#[derive(Debug)]
pub enum MqttSinkError {
    Client(rumqttc::ClientError),
    Json(serde_json::Error),
    Connection(rumqttc::ConnectionError),
}

impl fmt::Display for MqttSinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MqttSinkError::Client(e) => write!(f, "MQTT Client error: {e}"),
            MqttSinkError::Json(e) => write!(f, "JSON serialization error: {e}"),
            MqttSinkError::Connection(e) => write!(f, "MQTT Connection error: {e}"),
        }
    }
}

impl std::error::Error for MqttSinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MqttSinkError::Client(e) => Some(e),
            MqttSinkError::Json(e) => Some(e),
            MqttSinkError::Connection(e) => Some(e),
        }
    }
}

impl From<rumqttc::ClientError> for MqttSinkError {
    fn from(e: rumqttc::ClientError) -> Self {
        Self::Client(e)
    }
}

impl From<serde_json::Error> for MqttSinkError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

/* ---------- payload struct for JSON serialization ------------ */

#[derive(Serialize)]
struct MqttPayload {
    timestamp_ms: u64,
    value: f32,
}

/* ---------- sink struct -------------------------------------- */

pub struct MqttSink {
    client: Client,
    base_topic: String,
}

impl MqttSink {
    /// Establishes a connection to the MQTT broker.
    pub fn new(
        broker_host: &str,
        broker_port: u16,
        client_id: &str,
        base_topic: &str,
    ) -> Result<Self, MqttSinkError> {
        let mut mqttoptions = MqttOptions::new(client_id, broker_host, broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        // `Client` is for publishing, `Connection` handles the network loop
        let (client, mut connection) = Client::new(mqttoptions, 10);

        // Spawn a thread to process the connection event loop
        // This is crucial for keeping the connection alive and handling acknowledgements
        thread::spawn(move || {
            // This loop call is blocking, which is why it's in a thread.
            // It returns an error if the connection is lost.
            for (i, notification) in connection.iter().enumerate() {
                println!("Notification = {:?}, count = {}", notification, i);
            }
            println!("MQTT connection thread finished.");
        });

        Ok(Self {
            client,
            base_topic: base_topic.to_string(),
        })
    }
}

// Implement Drop to gracefully disconnect when the Sink is no longer needed
impl Drop for MqttSink {
    fn drop(&mut self) {
        // Best effort to disconnect
        let _ = self.client.disconnect();
    }
}


impl Sink for MqttSink {
    type Error = MqttSinkError;

    /// Stores a SensorSample by publishing each sensor reading to a unique MQTT topic.
    /// Topic structure: `base_topic/sensor_id/sensor_type`
    /// Payload format: JSON `{"timestamp_ms": 123456789, "value": 23.5}`
    fn store(&mut self, r: &SensorSample) -> Result<(), Self::Error> {
        for sensor in &r.sensors {
            // 1. Determine the topic for this specific sensor reading
            let sensor_type_str = match sensor.sensor_type {
                application::SensorType::Temperature => "temperature",
                application::SensorType::Humidity => "humidity",
                application::SensorType::Pressure => "pressure",
            };

            let topic = format!(
                "{}/{:?}/{}",
                self.base_topic, sensor.sensor_id, sensor_type_str
            );

            // 2. Create the JSON payload
            let payload = MqttPayload {
                timestamp_ms: r.timestamp_ms,
                value: sensor.value,
            };
            let payload_json = serde_json::to_string(&payload)?;

            // 3. Publish the message
            self.client.publish(
                &topic,
                QoS::AtLeastOnce, // QoS 1 is a good choice for sensor data
                false,            // `retain` flag, usually false
                payload_json.as_bytes(),
            )?;

            println!("Published to topic '{}'", topic);
        }

        Ok(())
    }
}