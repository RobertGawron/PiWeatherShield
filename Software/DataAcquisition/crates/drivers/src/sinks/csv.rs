#![cfg(feature = "std")]

use super::{SensorSample, Sink};
use csv::Writer;
use std::fmt;
use std::io::{BufWriter, Write};

/* ---------- error type --------------------------------------- */

#[derive(Debug)]
pub enum CsvSinkError {
    Csv(csv::Error),
    Io(std::io::Error),
}

impl fmt::Display for CsvSinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvSinkError::Csv(e) => write!(f, "CSV error: {e}"),
            CsvSinkError::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for CsvSinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CsvSinkError::Csv(e) => Some(e),
            CsvSinkError::Io(e) => Some(e),
        }
    }
}

impl From<csv::Error> for CsvSinkError {
    fn from(e: csv::Error) -> Self {
        Self::Csv(e)
    }
}

impl From<std::io::Error> for CsvSinkError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/* ---------- sink struct -------------------------------------- */

pub struct CsvSink<W: Write> {
    writer: Writer<W>,
}

impl CsvSink<BufWriter<std::fs::File>> {
    pub fn open(path: &str) -> Result<Self, CsvSinkError> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;

        let mut sink = Self::from_writer(BufWriter::new(file));
        // Write header only if file is empty/new
        sink.write_header()?;
        Ok(sink)
    }
}

impl<W: Write> CsvSink<W> {
    pub fn from_writer(writer: W) -> Self {
        let wtr = Writer::from_writer(writer);
        Self { writer: wtr }
    }

    fn write_header(&mut self) -> Result<(), CsvSinkError> {
        self.writer
            .write_record(&["timestamp_ms", "sensor_readings"])?;
        Ok(())
    }
}

impl<W: Write + std::marker::Send> Sink for CsvSink<W> {
    type Error = CsvSinkError;

    fn store(&mut self, r: &SensorSample) -> Result<(), Self::Error> {
        let timestamp = r.timestamp_ms.to_string();

        // Build sensor data string
        let sensor_data = r
            .sensors
            .iter()
            .map(|sensor| {
                let type_char = match sensor.sensor_type {
                    application::SensorType::Temperature => "T",
                    application::SensorType::Humidity => "H",
                    application::SensorType::Pressure => "P",
                };
                format!("{:?}:{}:{}", sensor.sensor_id, type_char, sensor.value)
            })
            .collect::<Vec<_>>()
            .join(";");

        self.writer.write_record(&[timestamp, sensor_data])?;
        self.writer.flush()?;
        Ok(())
    }
}
