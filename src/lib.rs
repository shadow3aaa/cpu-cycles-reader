//! This is only for reading `CpuCycles` specialization, not a complete package of [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
//!
//! Example:
//! ```
//! use std::{fs, time::{Duration, Instant}};
//! use cpu_cycles_reader::{Cycles, CyclesReader, CyclesInstant};
//!
//! let reader = CyclesReader::new().unwrap();
//!
//! let now = Instant::now();
//! let cycles_former = reader.read().unwrap();
//! let cycles_former = cycles_former.get(&7).unwrap(); // get cycles
//!
//! // The cpu has performed some operations, here we record cpu7
//!
//! let dur = Instant::now() - now;
//! let cycles_later = reader.read().unwrap();
//! let cycles_later = cycles_later.get(&7).unwrap(); // get cycles
//!
//! let cycles = *cycles_later - *cycles_former; // Calculate difference
//! // NOTE: There is no need to calculate the difference as a value within 1 second, there is such logic inside Cycles::as_usage() or Cycles::as_diff()
//!
//! let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", 7);
//! let cur_freq = fs::read_to_string(&path).unwrap();
//! let cur_freq = cur_freq.parse().unwrap();
//! let freq_cycles = Cycles::from_khz(cur_freq);
//!
//! let usage = cycles.as_usage(dur, freq_cycles).unwrap();
//! println!("{:.2}", usage);
//! ```
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_panics_doc, clippy::module_name_repetitions)]
#![cfg(any(target_os = "linux", target_os = "android"))]
mod cycles;
mod error;
pub mod ffi;
mod instant;

use std::ptr;

use ffi::CyclesReaderRaw;
use libc::c_int;

pub use cycles::Cycles;
pub use error::{Error, Result};
pub use instant::CyclesInstant;

#[derive(Debug)]
pub struct CyclesReader {
    raw_ptr: *mut CyclesReaderRaw,
}

impl Drop for CyclesReader {
    fn drop(&mut self) {
        unsafe {
            ffi::disableCyclesReader(self.raw_ptr);
            ffi::destroyCyclesReader(self.raw_ptr);
        }
        self.raw_ptr = ptr::null_mut();
    }
}

impl CyclesReader {
    /// # Errors
    ///
    /// If there is an error when calling the syscall, it will return an error
    pub fn new() -> Result<Self> {
        let cpus = c_int::try_from(num_cpus::get_physical())?;
        let cpus: Vec<_> = (0..cpus).collect();
        let cpus_ptr = cpus.as_ptr();

        let raw_ptr = unsafe {
            let ptr = ffi::createCyclesReader(cpus_ptr, cpus.len());
            ffi::enableCyclesReader(ptr);
            ptr
        };

        if raw_ptr.is_null() {
            return Err(Error::FailedToCreate);
        }

        Ok(Self { raw_ptr })
    }

    /// # Errors
    ///
    /// If there is an error when calling the syscall, it will return an error
    pub fn instant(&mut self, cpu: c_int) -> Result<CyclesInstant> {
        let raw = unsafe { ffi::readCyclesReader(self.raw_ptr, cpu) };

        if raw == -1 {
            Err(Error::FailedToRead)
        } else {
            let cycles = Cycles::new(raw);
            let instant = CyclesInstant::new(cpu, cycles);
            Ok(instant)
        }
    }
}
