//!  Used to handle all data sinks when there is new meassuqrement to store or show to user.
//!
//! This module provides a flexible system for collecting sensor readings and distributing
//! them to multiple output sinks (CSV files, stdout, OLED displays, etc.) without using
//! dynamic allocation.

use application::{Reading, Sink};
use arrayvec::ArrayVec;

/// A collector that distributes sensor readings to multiple sinks.
///
/// Uses `ArrayVec` for stack-based storage, making it suitable for embedded systems
/// or other environments where dynamic allocation should be avoided.
///
/// # Type Parameters
/// * `T` - The sink type (must implement `Sink`)
/// * `N` - The maximum number of sinks (compile-time constant)
///
/// # Example
/// ```ignore
/// let mut collector: SensorDataCollector<MySink, 3> = SensorDataCollector::new();
/// collector.add_sink(MySink::new())?;
/// collector.distribute(&reading)?;
/// ```
pub struct SensorDataCollector<T: Sink, const N: usize> {
    sinks: ArrayVec<T, N>,
}

impl<T: Sink, const N: usize> SensorDataCollector<T, N> {
    /// Creates a new empty collector.
    ///
    /// # Example
    /// ```ignore
    /// let collector: SensorDataCollector<MySink, 5> = SensorDataCollector::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sinks: ArrayVec::new(),
        }
    }

    /// Adds a sink to the collector.
    ///
    /// # Errors
    /// Returns the sink back if the collector is full (already contains `N` sinks).
    ///
    /// # Example
    /// ```ignore
    /// let mut collector = SensorDataCollector::<MySink, 3>::new();
    /// collector.add_sink(MySink::new())?;
    /// ```
    pub fn add_sink(&mut self, sink: T) -> Result<(), T> {
        self.sinks.try_push(sink).map_err(|e| e.element())
    }

    /// Distributes a reading to all sinks, collecting all potential errors.
    ///
    /// This method attempts to store the reading in all sinks, even if some fail.
    /// All errors are collected and returned with the index of the sink that failed.
    ///
    /// # Errors
    /// Returns a vector of `(index, error)` tuples for each sink that failed.
    /// If all sinks succeed, returns `Ok(())`.
    ///
    /// # Example
    /// ```ignore
    /// match collector.distribute(&reading) {
    ///     Ok(()) => println!("All sinks updated successfully"),
    ///     Err(errors) => {
    ///         for (idx, err) in errors {
    ///             eprintln!("Sink {} failed: {}", idx, err);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn distribute(&mut self, reading: &Reading) -> Result<(), Vec<(usize, T::Error)>> {
        let errors: Vec<_> = self
            .sinks
            .iter_mut()
            .enumerate()
            .filter_map(|(i, sink)| sink.store(reading).err().map(|e| (i, e)))
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T: Sink, const N: usize> Default for SensorDataCollector<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
