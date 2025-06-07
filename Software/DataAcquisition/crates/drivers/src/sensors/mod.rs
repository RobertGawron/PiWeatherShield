//! Combined module for concrete sensor drivers

pub mod bme280;
pub mod si7021;

pub use bme280::Bme280Sensor;
pub use si7021::Si7021Sensor;
