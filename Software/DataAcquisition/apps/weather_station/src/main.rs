use linux_embedded_hal::{Delay, I2cdev};
use ssd1306::I2CDisplayInterface;

use application::{SensorSample, Sensor, Measurement, SensorType};
use domain::SensorDataCollector;
use drivers::{Bme280Sensor, CsvSink, OledDisplay, Si7021Sensor, StdOutSink, MqttSink};
use ssd1306::prelude::I2CInterface;

use embedded_hal::delay::DelayNs;

use application::AnySink;

fn get_timestamp_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize I2C bus
    let i2c = I2cdev::new("/dev/i2c-1").map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Initialize OLED display
    let interface = I2CDisplayInterface::new(i2c);
    let display_sink =
        OledDisplay::new(interface).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let broker_host = "192.168.1.44"; // Use the IP of the Pi running Docker
    let broker_port = 1883;
    let client_id = "pi-weather-station";
    let base_topic = "weather";

    println!("Connecting to MQTT broker at {}:{}...", broker_host, broker_port);
    let mqtt_sink = MqttSink::new(broker_host, broker_port, client_id, base_topic)?;
    println!("Successfully connected to MQTT broker.");

    // Initialize sensors
    // BME280 uses the same I2C bus, so we need to clone it or open a new handle
    let i2c_bme =
        I2cdev::new("/dev/i2c-1").map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let mut bme_sensor = Bme280Sensor::new_secondary(i2c_bme)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let i2c_si =
        I2cdev::new("/dev/i2c-1").map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let mut si_sensor = Si7021Sensor::new(i2c_si); // Si7021Sensor::new does not return Result, so no map_err needed here.

    let stdout_sink = StdOutSink;

    let mut delay = Delay;
    let csv_sink = CsvSink::open("readings.csv")?;

    let mut sensor_collector: SensorDataCollector<
        AnySink<
            CsvSink<std::io::BufWriter<std::fs::File>>,
            StdOutSink,
            OledDisplay<I2CInterface<I2cdev>>,
            MqttSink<>,
        >,
        4,
    > = SensorDataCollector::new();

    // Add sinks wrapped in AnySink enum
    sensor_collector
        .add_sink(AnySink::Csv(csv_sink))
        .map_err(|_| "Failed to add CSV sink")?;
    sensor_collector
        .add_sink(AnySink::Stdout(stdout_sink))
        .map_err(|_| "Failed to add stdout sink")?;
    sensor_collector
        .add_sink(AnySink::Oled(display_sink))
        .map_err(|_| "Failed to add OLED sink")?;
    sensor_collector
        .add_sink(AnySink::Mqtt(mqtt_sink))
        .map_err(|_| "Failed to add MQTT sink")?;

    loop {
        // Read data from sensors
        let bme_data_raw = bme_sensor
            .read()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let si_data_raw = si_sensor
            .read()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let mut reading = SensorSample::new(get_timestamp_ms());

        // Add BME280 sensor readings
        if let Some(temp) = bme_data_raw.get_temperature(Some(application::SensorId::Bme280)) {
            reading
                .add_sensor(Measurement {
                    sensor_type: SensorType::Temperature,
                    value: temp,
                    unit: application::Unit::Celsius,
                    sensor_id: application::SensorId::Bme280,
                })
                .ok();
        }
        if let Some(humid) = bme_data_raw.get_humidity(Some(application::SensorId::Bme280)) {
            reading
                .add_sensor(Measurement {
                    sensor_type: SensorType::Humidity,
                    value: humid,
                    unit: application::Unit::Percent,
                    sensor_id: application::SensorId::Bme280,
                })
                .ok();
        }
        if let Some(pres) = bme_data_raw.get_pressure() {
            reading
                .add_sensor(Measurement {
                    sensor_type: SensorType::Pressure,
                    value: pres,
                    unit: application::Unit::HectoPascal,
                    sensor_id: application::SensorId::Bme280,
                })
                .ok();
        }

        // Add Si7021 sensor readings
        if let Some(temp) = si_data_raw.get_temperature(Some(application::SensorId::Si7021)) {
            reading
                .add_sensor(Measurement {
                    sensor_type: SensorType::Temperature,
                    value: temp,
                    unit: application::Unit::Celsius,
                    sensor_id: application::SensorId::Si7021,
                })
                .ok();
        }
        if let Some(humid) = si_data_raw.get_humidity(Some(application::SensorId::Si7021)) {
            reading
                .add_sensor(Measurement {
                    sensor_type: SensorType::Humidity,
                    value: humid,
                    unit: application::Unit::Percent,
                    sensor_id: application::SensorId::Si7021,
                })
                .ok();

            // Distribute reading to all sinks
            sensor_collector.distribute(&reading).map_err(|errors| {
                let error_msgs: Vec<String> = errors
                    .into_iter()
                    .map(|(idx, err)| format!("Sink {} failed: {}", idx, err))
                    .collect();
                format!("Distribution failed: {}", error_msgs.join(", "))
            })?;
        }

        // Delay for a period
        delay.delay_ms(5000_u32);
    }
}
