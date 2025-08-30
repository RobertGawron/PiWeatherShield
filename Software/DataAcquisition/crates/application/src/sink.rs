use crate::sensor_data::SensorSample;

/// A trait for anything that can store a sensor reading.
pub trait Sink: Send {
    // Define the associated error type for this sink.
    type Error: std::error::Error + Send + Sync + 'static;
    fn store(&mut self, r: &SensorSample) -> Result<(), Self::Error>;
}

/// A type-erased sink that can hold different concrete sink implementations.
///
/// This enum allows storing heterogeneous sink types in the same collection without
/// dynamic allocation by using generics.
#[derive(Debug)]
pub enum AnySink<CSV, STDOUT, OLED, MQTT>
where
    CSV: Sink,
    STDOUT: Sink,
    OLED: Sink,
    MQTT: Sink, // Add the trait bound for the new sink
{
    /// A CSV file sink, stores measurements to CSV file
    Csv(CSV),
    /// A standard output sink, useful for fast check what the measurements and if the app is running
    Stdout(STDOUT),
    /// An OLED display sink, shows to user the latest measurement.
    Oled(OLED),
    /// An MQTT sink, publishes measurements to a broker.
    Mqtt(MQTT), // Add the new variant
}

impl<CSV, STDOUT, OLED, MQTT> Sink for AnySink<CSV, STDOUT, OLED, MQTT>
where
    CSV: Sink,
    STDOUT: Sink,
    OLED: Sink,
    MQTT: Sink, // Add the trait bound here as well
{
    // The error type now includes the MQTT sink's error type
    type Error = AnySinkError<CSV::Error, STDOUT::Error, OLED::Error, MQTT::Error>;

    fn store(&mut self, reading: &SensorSample) -> Result<(), Self::Error> {
        match self {
            AnySink::Csv(sink) => sink.store(reading).map_err(AnySinkError::Csv),
            AnySink::Stdout(sink) => sink.store(reading).map_err(AnySinkError::Stdout),
            AnySink::Oled(sink) => sink.store(reading).map_err(AnySinkError::Oled),
            AnySink::Mqtt(sink) => sink.store(reading).map_err(AnySinkError::Mqtt), // Handle the new variant
        }
    }
}

/// Error type that preserves the original error from each sink type.
///
/// This allows for detailed error handling while maintaining type safety
/// and avoiding dynamic allocation.
#[derive(Debug)]
pub enum AnySinkError<C, S, O, M> { // Add a generic parameter for the MQTT error
    /// Error from CSV sink
    Csv(C),
    /// Error from stdout sink
    Stdout(S),
    /// Error from OLED sink
    Oled(O),
    /// Error from MQTT sink
    Mqtt(M), // Add the new variant
}

impl<C, S, O, M> core::fmt::Display for AnySinkError<C, S, O, M>
where
    C: core::fmt::Display,
    S: core::fmt::Display,
    O: core::fmt::Display,
    M: core::fmt::Display, // Add the Display bound for the MQTT error
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AnySinkError::Csv(e) => write!(f, "CSV sink error: {}", e),
            AnySinkError::Stdout(e) => write!(f, "Stdout sink error: {}", e),
            AnySinkError::Oled(e) => write!(f, "OLED sink error: {}", e),
            AnySinkError::Mqtt(e) => write!(f, "MQTT sink error: {}", e), // Handle the new variant
        }
    }
}

impl<C, S, O, M> std::error::Error for AnySinkError<C, S, O, M>
where
    C: std::error::Error + 'static,
    S: std::error::Error + 'static,
    O: std::error::Error + 'static,
    M: std::error::Error + 'static, // Add the Error bound for the MQTT error
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnySinkError::Csv(e) => Some(e),
            AnySinkError::Stdout(e) => Some(e),
            AnySinkError::Oled(e) => Some(e),
            AnySinkError::Mqtt(e) => Some(e), // Handle the new variant
        }
    }
}