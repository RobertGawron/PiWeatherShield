use crate::sensor_data::Reading;

/// Represents a sensor connected to the PCB that provides environmental data.
///
/// This trait abstracts over different types of sensors (e.g., temperature, humidity,
/// atmospheric pressure) that can be used to collect readings from the environment.
///
pub trait Sensor {
    /// The error type returned if reading from the sensor fails.
    type Error;

    /// Reads a value from the sensor.
    fn read(&mut self) -> Result<Reading, Self::Error>;
}
