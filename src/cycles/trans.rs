//! Conversion between Cycles struct and Frequency

use libc::c_longlong as c_ll;

use super::Cycles;

impl Cycles {
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1000), Cycles::from_hz(1000));
    /// ```
    #[must_use]
    pub fn from_hz(h: c_ll) -> Self {
        Self { raw: h }
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1000), Cycles::from_khz(1));
    /// ```
    #[must_use]
    pub fn from_khz(k: c_ll) -> Self {
        Self { raw: k * 1000 }
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(2_000_000), Cycles::from_mhz(2));
    /// ```
    #[must_use]
    pub fn from_mhz(m: c_ll) -> Self {
        Self { raw: m * 1_000_000 }
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(3_000_000_000), Cycles::from_ghz(3));
    /// ```
    #[must_use]
    pub fn from_ghz(g: c_ll) -> Self {
        Self {
            raw: g * 1_000_000_000,
        }
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1000).as_hz(), 1000);
    /// ```
    #[must_use]
    pub fn as_hz(&self) -> c_ll {
        self.raw
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1_000_000).as_khz(), 1000);
    /// ```
    #[must_use]
    pub fn as_khz(&self) -> c_ll {
        self.raw / 1000
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1_000_000_000).as_mhz(), 1000);
    /// ```
    #[must_use]
    pub fn as_mhz(&self) -> c_ll {
        self.raw / 1_000_000
    }

    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1_000_000_000_000).as_ghz(), 1000);
    /// ```
    #[must_use]
    pub fn as_ghz(&self) -> c_ll {
        self.raw / 1_000_000_000
    }
}
