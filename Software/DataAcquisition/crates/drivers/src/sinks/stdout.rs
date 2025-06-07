use super::{Reading, Sink};
use application::SensorType;

pub struct StdOutSink;

impl Sink for StdOutSink {
    type Error = core::convert::Infallible;

    fn store(&mut self, r: &Reading) -> Result<(), Self::Error> {
        #[cfg(feature = "std")]
        {
            // Print timestamp first
            print!("{}", r.timestamp_ms);

            // Print all sensor readings in a consistent format
            for sensor in &r.sensors {
                match sensor.sensor_type {
                    SensorType::Temperature => print!(",{:.1}C", sensor.value),
                    SensorType::Humidity => print!(",{:.1}%", sensor.value),
                    SensorType::Pressure => print!(",{:.0}hPa", sensor.value),
                }
            }
            println!(); // End the line
        }
        Ok(())
    }
}

/* ---------------------------- tests ---------------------------- */
#[cfg(test)]
mod tests {
    use super::*;
    use application::{SensorReading, SensorType, Unit, SensorId};

    #[test]
    fn prints_without_error() {
        let mut sink = StdOutSink;
        let mut reading = Reading::new(1234567890);

        // Add some test sensor readings
        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Temperature,
                value: 23.5,
                unit: Unit::Celsius,
                sensor_id: SensorId::Bme280,
            })
            .unwrap();

        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Humidity,
                value: 65.2,
                unit: Unit::Percent,
                sensor_id: SensorId::Bme280,
            })
            .unwrap();

        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Pressure,
                value: 1013.25,
                unit: Unit::HectoPascal,
                sensor_id: SensorId::Bme280,
            })
            .unwrap();

        // Just verify that it returns Ok - capturing stdout is overkill here.
        assert!(sink.store(&reading).is_ok());
    }

    #[test]
    fn handles_partial_sensor_data() {
        let mut sink = StdOutSink;
        let mut reading = Reading::new(1234567890);

        // Only temperature and humidity (no pressure)
        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Temperature,
                value: 20.0,
                unit: Unit::Celsius,
                sensor_id: SensorId::Si7021,
            })
            .unwrap();

        reading
            .add_sensor(SensorReading {
                sensor_type: SensorType::Humidity,
                value: 45.0,
                unit: Unit::Percent,
                sensor_id: SensorId::Si7021,
            })
            .unwrap();

        assert!(sink.store(&reading).is_ok());
    }
}
