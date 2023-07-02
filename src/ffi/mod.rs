use libc::{c_int, c_longlong as c_ll, size_t};

extern "C" {
    pub(super) fn createCyclesReader(cpus: *const c_int, num_cpus: size_t) -> *mut CyclesReaderRaw; // Constructor
    pub(super) fn destroyCyclesReader(reader: *mut CyclesReaderRaw); // Destructor
    pub(super) fn enableCyclesReader(reader: *mut CyclesReaderRaw);
    pub(super) fn disableCyclesReader(reader: *mut CyclesReaderRaw);
    pub(super) fn readCyclesReader(reader: *mut CyclesReaderRaw) -> *mut c_ll;
}

#[repr(C)]
pub(super) struct CyclesReaderRaw {
    pub size: size_t,
    pub cpus: *mut c_int,
}
