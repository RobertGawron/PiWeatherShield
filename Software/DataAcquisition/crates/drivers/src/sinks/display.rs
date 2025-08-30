//! OLED display sink - renders the latest `SensorSample` on a 12832 SSD1306
use core::fmt::{self, Write as _};

use application::Sink;
use application::{SensorSample, Measurement, SensorType};
use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_graphics::{
    mono_font::{ascii::FONT_9X15, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use heapless::String;
use ssd1306::mode::DisplayConfig;
use ssd1306::{
    mode::BufferedGraphicsMode,
    prelude::{DisplayRotation, DisplaySize128x32},
    Ssd1306,
};

pub enum OledDisplayError {
    Display(DisplayError),
    Format(fmt::Error),
}

// Manual implementation of Debug for OledDisplayError
impl fmt::Debug for OledDisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // DisplayError does not implement Debug, so we provide a generic description
            Self::Display(_) => f
                .debug_tuple("OledDisplayError::Display")
                .field(&"Display interface error")
                .finish(),
            Self::Format(e) => f.debug_tuple("OledDisplayError::Format").field(e).finish(),
        }
    }
}

impl fmt::Display for OledDisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // DisplayError does not implement Display, so we provide a generic description
            Self::Display(_) => write!(f, "Display error:"),
            Self::Format(e) => write!(f, "Format error: {}", e),
        }
    }
}

// This implementation is needed for `?` operator when `std` is enabled
#[cfg(feature = "std")]
impl std::error::Error for OledDisplayError {}

impl From<DisplayError> for OledDisplayError {
    fn from(e: DisplayError) -> Self {
        Self::Display(e)
    }
}
impl From<fmt::Error> for OledDisplayError {
    fn from(e: fmt::Error) -> Self {
        Self::Format(e)
    }
}

pub struct OledDisplay<DI>
where
    DI: WriteOnlyDataCommand,
{
    oled: Ssd1306<DI, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>>,
}

impl<DI> OledDisplay<DI>
where
    DI: WriteOnlyDataCommand,
{
    pub fn new(interface: DI) -> Result<Self, OledDisplayError> {
        let mut oled = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate180)
            .into_buffered_graphics_mode();

        // Corrected error mapping
        oled.init().map_err(OledDisplayError::Display)?;

        Ok(Self { oled })
    }
}

impl<DI> Sink for OledDisplay<DI>
where
    DI: WriteOnlyDataCommand + std::marker::Send,
{
    type Error = OledDisplayError;

    fn store(&mut self, reading: &SensorSample) -> Result<(), Self::Error> {
        // Use the proper helper methods from SensorSample struct
        let bme_temp = reading.get_temperature(Some(application::SensorId::Bme280)).unwrap_or(0.0);
        let bme_humidity = reading.get_humidity(Some(application::SensorId::Bme280)).unwrap_or(0.0);
        let si_temp = reading.get_temperature(Some(application::SensorId::Si7021)).unwrap_or(0.0);
        let si_humidity = reading.get_humidity(Some(application::SensorId::Si7021)).unwrap_or(0.0);
        let pressure = reading.get_pressure().unwrap_or(0.0);

        // Format the display text
        let mut buf = String::<64>::new();

        writeln!(&mut buf, "{:.1}/{:.1}C", bme_temp, si_temp)?;
        writeln!(&mut buf, "{:.0}hPa {:.1}%", pressure, si_humidity)?;

        // Clear display and render
        self.oled.clear_buffer();
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_9X15)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(&buf, Point::zero(), style, Baseline::Top)
            .draw(&mut self.oled)
            .ok();

        self.oled.flush().map_err(OledDisplayError::Display)?;
        Ok(())
    }
}

#[cfg(test)]
mod visual_tests {
    use super::*;
    use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
    use embedded_graphics::pixelcolor::BinaryColor;
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
    use std::sync::{Arc, Mutex};

    // Mock display interface that captures the framebuffer data
    struct MockDisplayInterface {
        // Shared buffer to capture display data
        buffer: Arc<Mutex<Vec<u8>>>,
        // Track if we're receiving data (vs commands)
        // data_mode: bool,
        write_pos: usize,
    }

    impl MockDisplayInterface {
        fn new() -> Self {
            Self {
                buffer: Arc::new(Mutex::new(vec![0; 512])), // 128x32 / 8 = 512 bytes
                //data_mode: false,
                write_pos: 0,
            }
        }

        fn get_buffer_clone(&self) -> Arc<Mutex<Vec<u8>>> {
            Arc::clone(&self.buffer)
        }
    }

    impl WriteOnlyDataCommand for MockDisplayInterface {
        fn send_commands(&mut self, cmds: DataFormat<'_>) -> Result<(), DisplayError> {
            // If this is a command that resets the write pointer, reset write_pos
            // For simplicity, reset on any command (real code should parse commands)
            self.write_pos = 0;
            Ok(())
        }

        fn send_data(&mut self, data: DataFormat<'_>) -> Result<(), DisplayError> {
            if let DataFormat::U8(slice) = data {
                let mut buffer = self.buffer.lock().unwrap();
                let end = (self.write_pos + slice.len()).min(buffer.len());
                let count = end - self.write_pos;
                buffer[self.write_pos..end].copy_from_slice(&slice[..count]);
                self.write_pos += count;
            }
            Ok(())
        }
    }

    // Helper to convert the captured buffer to a SimulatorDisplay
    fn buffer_to_simulator_display(
        buffer: &[u8],
        width: u32,
        height: u32,
    ) -> SimulatorDisplay<BinaryColor> {
        let mut sim_display = SimulatorDisplay::new(Size::new(width, height));

        // SSD1306 buffer format: each byte represents 8 vertical pixels
        // The buffer is organized in pages (rows of 8 pixels)
        let pages = height / 8;
        let bytes_per_page = width;

        for page in 0..pages {
            for col in 0..width {
                let byte_index = (page * bytes_per_page + col) as usize;
                if byte_index < buffer.len() {
                    let byte = buffer[byte_index];

                    draw_ssd1306_byte(&mut sim_display, col, page, byte, height);
                }
            }
        }

        sim_display
    }

    fn draw_ssd1306_byte(
        sim_display: &mut SimulatorDisplay<BinaryColor>,
        col: u32,
        page: u32,
        byte: u8,
        height: u32,
    ) {
        for bit in 0..8 {
            let y = page * 8 + bit;
            if y < height {
                let pixel_on = (byte & (1 << bit)) != 0;
                let color = if pixel_on {
                    BinaryColor::On
                } else {
                    BinaryColor::Off
                };
                let _ = Pixel(Point::new(col as i32, y as i32), color).draw(sim_display);
            }
        }
    }

    // Helper function to create test readings
    fn create_test_reading(
        bme_temp: f32,
        bme_humidity: f32,
        bme_pressure: f32,
        si_temp: f32,
        si_humidity: f32,
        timestamp_ms: u64,
    ) -> SensorSample {
        let mut reading = SensorSample::new(timestamp_ms);

        // Add BME280 readings
        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Temperature,
                value: bme_temp,
                unit: application::Unit::Celsius,
                sensor_id: application::SensorId::Bme280,
            })
            .ok();

        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Humidity,
                value: bme_humidity,
                unit: application::Unit::Percent,
                sensor_id: application::SensorId::Bme280,
            })
            .ok();

        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Pressure,
                value: bme_pressure,
                unit: application::Unit::HectoPascal,
                sensor_id: application::SensorId::Bme280,
            })
            .ok();

        // Add SI7021 readings
        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Temperature,
                value: si_temp,
                unit: application::Unit::Celsius,
                sensor_id: application::SensorId::Si7021,
            })
            .ok();

        reading
            .add_sensor(Measurement {
                sensor_type: SensorType::Humidity,
                value: si_humidity,
                unit: application::Unit::Percent,
                sensor_id: application::SensorId::Si7021,
            })
            .ok();

        reading
    }

    #[test]
    fn test_store_normal_conditions() {
        // Create mock interface and get a reference to its buffer
        let mock_interface = MockDisplayInterface::new();
        let buffer_ref = mock_interface.get_buffer_clone();

        // Create the actual OledDisplay with our mock
        let mut oled_display =
            OledDisplay::new(mock_interface).expect("Failed to create OledDisplay");

        // Create test data
        let reading = create_test_reading(
            23.5,    // BME280 temperature
            45.0,    // BME280 humidity
            1013.25, // BME280 pressure
            24.0,    // SI7021 temperature
            50.0,    // SI7021 humidity
            1000000, // timestamp
        );

        // Call the actual store method being tested
        oled_display
            .store(&reading)
            .expect("Failed to store reading");

        // Convert captured buffer to simulator display
        let buffer = buffer_ref.lock().unwrap();
        let sim_display = buffer_to_simulator_display(&buffer, 128, 32);

        // Save the display output
        let output_settings = OutputSettingsBuilder::new().scale(2).build();

        let image = sim_display.to_rgb_output_image(&output_settings);
        let png_filename = "test_output_normal_conditions.png";

        image.save_png(png_filename).expect("Failed to save image");
    }
}
