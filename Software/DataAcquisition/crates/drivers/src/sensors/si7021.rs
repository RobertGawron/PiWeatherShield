use linux_embedded_hal::Delay;
use si7021_t_rh as si7021;

use application::Sensor;
use application::{Reading, SensorReading, SensorType, Unit, SensorId};
use core::fmt; // Import fmt for manual Display/Debug implementations

// Define a new error type that wraps the si7021_t_rh error
pub enum Si7021Error<E> {
    /// Error from the underlying I2C bus
    I2c(E),
    /// Error from the si7021-t-rh driver
    Driver(si7021::error::Error<E>),
}

// Manual implementation of Debug for Si7021Error
impl<E: fmt::Debug> fmt::Debug for Si7021Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Si7021Error::I2c(e) => f.debug_tuple("Si7021Error::I2c").field(e).finish(),
            Si7021Error::Driver(e) => f.debug_tuple("Si7021Error::Driver").field(e).finish(),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> fmt::Display for Si7021Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Si7021Error::I2c(e) => write!(f, "I2C error: {}", e),
            // Fix: Use debug formatter for the inner driver error as it doesn't implement Display
            Si7021Error::Driver(e) => write!(f, "Si7021 driver error: {:?}", e),
        }
    }
}

// Conditionally implement std::error::Error for compatibility with Box<dyn std::error::Error>
#[cfg(feature = "std")]
impl<E> std::error::Error for Si7021Error<E> where E: std::error::Error + 'static {}

// Implement From trait to allow easy conversion from the driver's error
impl<E> From<si7021::error::Error<E>> for Si7021Error<E> {
    fn from(e: si7021::error::Error<E>) -> Self {
        Si7021Error::Driver(e)
    }
}

pub struct Si7021Sensor<I2C> {
    inner: si7021::Si7021<I2C, Delay>, // driver uses hal-1.0 traits
}

impl<I2C, E> Si7021Sensor<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            inner: si7021::Si7021::new(i2c, Delay),
        }
    }
}

impl<I2C, E> Sensor for Si7021Sensor<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug + core::fmt::Display + 'static, // Ensure E implements Debug and Display
{
    type Error = Si7021Error<E>; // Use the new error type
    fn read(&mut self) -> Result<Reading, Self::Error> {
        let t = self.inner.read_temperature().map_err(Si7021Error::Driver)?;
        let h = self
            .inner
            .read_relative_humidity()
            .map_err(Si7021Error::Driver)?;

        let mut reading = Reading::new(0);

        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Temperature,
                value: t,
                unit: Unit::Celsius,
                sensor_id: SensorId::Si7021,
            })
            .ok();
        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Humidity,
                value: h,
                unit: Unit::HectoPascal,
                sensor_id: SensorId::Si7021,
            })
            .ok();
        Ok(reading)
    }
}
