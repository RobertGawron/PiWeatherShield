use bme280_rs::{Bme280, Configuration, Oversampling, SensorMode};
use embedded_hal::i2c::I2c;
use linux_embedded_hal::Delay;

use application::Sensor;
use application::{SensorSample, Measurement, SensorType, Unit, SensorId};

/* --------------------------------------------------------------------- */
pub struct Bme280Sensor<I2C> {
    inner: Bme280<I2C, Delay>,
}

impl<I2C, E> Bme280Sensor<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Secondary-address constructor (0x76).  
    /// Does the mandatory `init()` and puts the chip in "Normal" mode.
    /// todo this is not secondary but primary
    pub fn new_secondary(i2c: I2C) -> Result<Self, E> {
        let mut drv = Bme280::new_with_address(i2c, 0x77, Delay);
        drv.init()?; // soft-reset
        drv.set_sampling_configuration(
            Configuration::default()
                .with_temperature_oversampling(Oversampling::Oversample1)
                .with_pressure_oversampling(Oversampling::Oversample1)
                .with_humidity_oversampling(Oversampling::Oversample1)
                .with_sensor_mode(SensorMode::Normal),
        )?;
        Ok(Self { inner: drv })
    }
}

/* --------------------------------------------------------------------- */
impl<I2C, E> Sensor for Bme280Sensor<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = E;

    fn read(&mut self) -> Result<SensorSample, Self::Error> {
        // Every call returns Result<Option<f32>, E>
        let t = self.inner.read_temperature()?.unwrap_or_default();
        let h = self.inner.read_humidity()?.unwrap_or_default();
        let p = self.inner.read_pressure()?.unwrap_or_default() / 100.0;

        let mut reading = SensorSample::new(0); // Timestamp should be set by application layer

        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Temperature,
                value: t,
                unit: Unit::Celsius,
                sensor_id: SensorId::Bme280,
            })
            .ok();

        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Humidity,
                value: h,
                unit: Unit::Percent,
                sensor_id: SensorId::Bme280,
            })
            .ok();

        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Pressure,
                value: p,
                unit: Unit::HectoPascal,
                sensor_id: SensorId::Bme280,
            })
            .ok();

        Ok(reading)
    }
}
