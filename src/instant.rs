use std::ops::Sub;

use libc::c_int;

use crate::{Cycles, Error, Result};

#[derive(Debug, Clone, Copy)]
pub struct CyclesInstant {
    cpu: c_int,
    raw: Cycles,
}

impl CyclesInstant {
    pub(crate) const fn new(cpu: c_int, raw: Cycles) -> Self {
        Self { cpu, raw }
    }

    /// Calculate the number of cpu cycles between two recordings
    ///
    /// # Panics
    ///
    /// If the two records are not the same cpu, an error will be returned.
    #[must_use]
    pub fn cycles_since(&self, other: Self) -> Cycles {
        self.cycles_since_checked(other).unwrap()
    }

    /// Calculate the number of cpu cycles between two recordings
    ///
    /// # Errors
    ///
    /// If the two records are not the same cpu, an error will be returned
    pub fn cycles_since_checked(&self, other: Self) -> Result<Cycles> {
        if self.cpu == other.cpu {
            Ok(self.raw - other.raw)
        } else {
            Err(Error::InconsistentCore)
        }
    }
}

impl Sub for CyclesInstant {
    type Output = Cycles;

    fn sub(self, other: Self) -> Self::Output {
        self.cycles_since(other)
    }
}
