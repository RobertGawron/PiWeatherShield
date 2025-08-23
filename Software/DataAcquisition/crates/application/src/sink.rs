use crate::sensor_data::Reading;

/// A trait for anything that can store a sensor reading.
pub trait Sink: Send {
    type Error: Send + Sync + 'static;
    fn store(&mut self, r: &Reading) -> Result<(), Self::Error>;
}

/// A type-erased sink that can hold different concrete sink implementations.
///
/// This enum allows storing heterogeneous sink types in the same collection without
/// dynamic allocation.
#[derive(Debug)]
pub enum AnySink<CSV, STDOUT, OLED>
where
    CSV: Sink,
    STDOUT: Sink,
    OLED: Sink,
{
    /// A CSV file sink, stores measurements to CSV file
    Csv(CSV),
    /// A standard output sink, useful for fast check what the measurements and if the app is running
    Stdout(STDOUT),
    /// An OLED display sink, shows to user the latest measurement.
    Oled(OLED),
}

impl<CSV, STDOUT, OLED> Sink for AnySink<CSV, STDOUT, OLED>
where
    CSV: Sink,
    STDOUT: Sink,
    OLED: Sink,
{
    type Error = AnySinkError<CSV::Error, STDOUT::Error, OLED::Error>;

    fn store(&mut self, reading: &Reading) -> Result<(), Self::Error> {
        match self {
            AnySink::Csv(sink) => sink.store(reading).map_err(AnySinkError::Csv),
            AnySink::Stdout(sink) => sink.store(reading).map_err(AnySinkError::Stdout),
            AnySink::Oled(sink) => sink.store(reading).map_err(AnySinkError::Oled),
        }
    }
}

/// Error type that preserves the original error from each sink type.
///
/// This allows for detailed error handling while maintaining type safety
/// and avoiding dynamic allocation.
#[derive(Debug)]
pub enum AnySinkError<C, S, O> {
    /// Error from CSV sink
    Csv(C),
    /// Error from stdout sink
    Stdout(S),
    /// Error from OLED sink
    Oled(O),
}

impl<C, S, O> core::fmt::Display for AnySinkError<C, S, O>
where
    C: core::fmt::Display,
    S: core::fmt::Display,
    O: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AnySinkError::Csv(e) => write!(f, "CSV sink error: {}", e),
            AnySinkError::Stdout(e) => write!(f, "Stdout sink error: {}", e),
            AnySinkError::Oled(e) => write!(f, "OLED sink error: {}", e),
        }
    }
}

impl<C, S, O> std::error::Error for AnySinkError<C, S, O>
where
    C: std::error::Error + 'static,
    S: std::error::Error + 'static,
    O: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnySinkError::Csv(e) => Some(e),
            AnySinkError::Stdout(e) => Some(e),
            AnySinkError::Oled(e) => Some(e),
        }
    }
}
