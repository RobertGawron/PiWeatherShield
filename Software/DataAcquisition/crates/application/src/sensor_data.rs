use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    // Future: AirQuality, Light, etc.
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Celsius,
    Percent,
    HectoPascal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SensorId {
    Bme280,
    Si7021
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensorReading {
    pub sensor_type: SensorType,
    pub value: f32,
    pub unit: Unit,
    pub sensor_id: SensorId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Reading {
    pub timestamp_ms: u64,
    pub sensors: Vec<SensorReading, 16>, // Max 16 sensor readings
}

impl Reading {
    pub fn new(timestamp_ms: u64) -> Self {
        Self {
            timestamp_ms,
            sensors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, reading: SensorReading) -> Result<(), ()> {
        self.sensors.push(reading).map_err(|_| ())
    }

    // Helper methods for common queries
    pub fn get_temperature(&self, sensor_id: Option<SensorId>) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| {
                s.sensor_type == SensorType::Temperature
                    && sensor_id.map_or(true, |id| s.sensor_id == id)
            })
            .map(|s| s.value)
    }

    pub fn get_humidity(&self, sensor_id: Option<SensorId>) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| {
                s.sensor_type == SensorType::Humidity
                    && sensor_id.map_or(true, |id| s.sensor_id == id)
            })
            .map(|s| s.value)
    }

    pub fn get_pressure(&self) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| s.sensor_type == SensorType::Pressure)
            .map(|s| s.value)
    }

    pub fn get_all_temperatures(&self) -> impl Iterator<Item = &SensorReading> {
        self.sensors
            .iter()
            .filter(|s| s.sensor_type == SensorType::Temperature)
    }

    pub fn get_all_humidities(&self) -> impl Iterator<Item = &SensorReading> {
        self.sensors
            .iter()
            .filter(|s| s.sensor_type == SensorType::Humidity)
    }
}
