//! Base bindings to c code
//!
//! Unless you are really sure you have to use them, just use [`crate::CyclesReader`] for normal purpose

use libc::{c_int, c_longlong as c_ll, size_t};

extern "C" {
    /// Mock constructor
    ///
    /// This C function internally calls malloc to allocate a memory to construct CyclesReaderRaw, and returns a pointer to the heap
    ///
    /// If there is an error in the creation process, the memory will be free, and the pointer will be set to NULL to return, special attention should be paid
    pub fn createCyclesReader(cpus: *const c_int, num_cpus: size_t) -> *mut CyclesReaderRaw;

    /// Mock destructor
    ///
    /// This C function internally calls free to release the memory and sets the pointer to NULL
    pub fn destroyCyclesReader(reader: *mut CyclesReaderRaw);

    /// This C function calls ioctl to start recording Cycles, see [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
    pub fn enableCyclesReader(reader: *mut CyclesReaderRaw);

    /// This C function calls ioctl to stop recording Cycles, see [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
    pub fn disableCyclesReader(reader: *mut CyclesReaderRaw);

    /// This C function reads Cycles information by reading the file identifier, see [perf_event_read](https://www.man7.org/linux/man-pages/man2/perf_event_open.2.html)
    ///
    /// The returned array pointer is also allocated by malloc, consider calling [`libc::free`] to release to ensure memory safety, and remember to prevent dangling pointers
    ///
    /// NOTE: The length of the array is the number of CPUs used during construction. Consider using the `size` member of [`self::CyclesReaderRaw`] to determine the length of the array
    pub fn readCyclesReader(reader: *mut CyclesReaderRaw) -> *mut c_ll;
}

/// The Raw Reader Structure, corresponding to the same structure in C
#[repr(C)]
pub struct CyclesReaderRaw {
    pub size: size_t,
    cpus: *mut c_int,
}
