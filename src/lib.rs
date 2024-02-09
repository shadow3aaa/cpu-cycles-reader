//! This is only for reading `CpuCycles` specialization, not a complete package of [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
//!
//! Example:
//! ```ignore
//! use std::{fs, time::{Duration, Instant}, thread};
//! use cpu_cycles_reader::{Cycles, CyclesReader, CyclesInstant};
//!
//! let reader = CyclesReader::new().unwrap();
//! let record_1 = reader.instant(0).unwrap();
//!
//! thread::sleep(Duration::from_secs(1));
//!
//! let record_2 = reader.instant(0).unwrap();
//! let cycles = record_2 - record_1;
//!
//! println!("{cycles}");
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
    pub fn instant(&self, cpu: c_int) -> Result<CyclesInstant> {
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
