//! Only for reading CpuCycles specialization, not a complete package of [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
//! ⚠ Permission requirements: Make sure the program has root permissions
//!
//! Example:
//! ```ignore
//! use std::time::{Duration, Instant};
//! use cpu_cycles_reader::{Cycles, CyclesReader};
//! let reader = CyclesReader::new(&[7]).unwrap();
//! reader.enable();
//!
//! let now = Instant::now();
//! let cycles_former = reader.read().unwrap();
//! let cycles_former = cycles_former.get(&7).unwrap();
//!
//! // The cpu has performed some operations, we record cpu7
//!
//! let dur = Instant::now() - now;
//! let cycles_later = reader.read().unwrap();
//! let cycles_later = cycles_later.get(&7).unwrap();
//!
//! let cycles = *cycles_later - *cycles_former;
//! let usage = cycles.as_usage(dur, 7).unwrap();
//! println!("{:.2}", usage);
//! ```

mod cycles;
mod ffi;

use std::{collections::HashMap, ptr, slice};

use ffi::CyclesReaderRaw;
use libc::{c_int, c_void};

pub use cycles::Cycles;

pub struct CyclesReader {
    raw_ptr: *mut CyclesReaderRaw,
    cpus: Vec<c_int>,
}

impl Drop for CyclesReader {
    fn drop(&mut self) {
        unsafe { ffi::destroyCyclesReader(self.raw_ptr) } // ffi里面已经free，不需要rust调用free
        self.raw_ptr = ptr::null_mut();
    }
}

impl CyclesReader {
    /// Create CyclesReader
    /// ```ignore
    /// use cpu_cycles_reader::CyclesReader;
    /// let reader = CyclesReader::new(&[0, 1, 2, 3]).unwrap();
    /// ```
    pub fn new(cpus: &[c_int]) -> Result<Self, &'static str> {
        let cpus = cpus.to_vec();
        let cpus_ptr = cpus.as_ptr();

        let raw_ptr = unsafe { ffi::createCyclesReader(cpus_ptr, cpus.len()) };
        if raw_ptr.is_null() {
            return Err("Failed to create CyclesReader");
        }
        Ok(Self { raw_ptr, cpus })
    }

    /// Enable Cycles monitoring
    /// ```ignore
    /// use cpu_cycles_reader::CyclesReader;
    ///
    /// let reader = CyclesReader::new(&[0, 1, 2, 3]).unwrap();
    /// reader.enable();
    /// ```
    pub fn enable(&self) {
        unsafe {
            ffi::enableCyclesReader(self.raw_ptr);
        }
    }

    /// Disable Cycles monitoring
    /// ```ignore
    /// use cpu_cycles_reader::CyclesReader;
    ///
    /// let reader = CyclesReader::new(&[0, 1, 2, 3]).unwrap();
    /// reader.disable();
    /// ```
    pub fn disable(&self) {
        unsafe {
            ffi::disableCyclesReader(self.raw_ptr);
        }
    }

    /// Read the number of Cycles from start to present
    /// Return in the order of the cpu parameters of the constructor
    pub fn read(&self) ->Result<HashMap<c_int, Cycles>, &'static str> {
        let raw = unsafe { ffi::readCyclesReader(self.raw_ptr) };

        if raw.is_null() {
            return Err("CyclesReader failed to read");
        }

        let slice = unsafe { slice::from_raw_parts(raw, (*self.raw_ptr).size) };
        let map = self
            .cpus
            .iter()
            .zip(slice)
            .map(|(c, d)| (*c, Cycles::from(*d)))
            .collect(); // copied here

        // Free the array of ffi malloc
        unsafe { libc::free(raw as *mut c_void) }

        Ok(map)
    }
}
