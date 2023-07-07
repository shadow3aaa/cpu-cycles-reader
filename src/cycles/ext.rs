//! Expansion of Cycles struct

use std::{error::Error, time::Duration};

use libc::c_longlong as c_ll;

use super::Cycles;

impl Cycles {
    /// Returns the average number of cpu usage within a specified time
    ///
    /// May be >100%, because the current frequency read is only a suggested frequency, not a real frequency, but the cycles are real
    ///
    /// d: record the time of cycles
    ///
    /// f: frequencey of this cpu core as [`Cycles`]
    ///
    /// Suggestion: Read `/sys/devices/system/cpu/cpuX/cpufreq/scaling_cur_freq` to get the current frequency
    ///
    /// # Errors
    /// Divide by zero or fail to trans u128 to i64
    /// ```ignore
    /// use std::{fs, time::{Duration, Instant}};
    /// use cpu_cycles_reader::Cycles;
    ///
    /// let now = Instant::now();
    /// let cycles_former = Cycles::from_ghz(1); // Suppose it is 1ghz at this time
    ///
    /// // The cpu has performed some operations, assuming we are recording cpu7
    ///
    /// let dur = Instant::now() - now;
    /// let cycles_later = Cycles::from_ghz(2); // Suppose it is 2ghz at this time
    ///
    /// let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", 7);
    /// let cur_freq = fs::read_to_string(&path).unwrap();
    /// let cur_freq = cur_freq.parse().unwrap();
    /// let freq_cycles = Cycles::from_khz(cur_freq);
    ///
    /// let cycles = cycles_later - cycles_former;
    /// println!("{:.2}", cycles.as_usage(dur, freq_cycles).unwrap()); // Suppose you read cycles on cpu7
    /// ```
    #[allow(clippy::cast_precision_loss)]
    pub fn as_usage(&self, d: Duration, f: Self) -> Result<f64, Box<dyn Error>> {
        let hz = (self.raw * 1_000_000)
            .checked_div(c_ll::try_from(d.as_micros())?)
            .ok_or("Failed to div")?;
        let cur_hz = f.as_hz();
        Ok(hz as f64 / cur_hz as f64)
    }

    /// Similar to `as_usage`, but returns the difference from the current frequency [`Cycles`]
    ///
    /// For the same reason, diff may be negative
    ///
    /// d: record the time of cycles
    ///
    /// f: frequencey of this cpu core as [`Cycles`]
    ///
    /// Suggestion: Read `/sys/devices/system/cpu/cpuX/cpufreq/scaling_cur_freq` to get the current frequency
    ///
    /// # Errors
    /// Divide by zero
    /// ```ignore
    /// use std::{fs, time::{Duration, Instant}};
    /// use cpu_cycles_reader::Cycles;
    ///
    /// let now = Instant::now();
    /// let cycles_former = Cycles::from_ghz(1); // Suppose it is 1ghz at this time
    ///
    /// // The cpu has performed some operations, assuming we are recording the cpu7
    ///
    /// let dur = Instant::now() - now;
    /// let cycles_later = Cycles::from_ghz(2); // Suppose it is 2ghz at this time
    ///
    /// let cycles = cycles_later - cycles_former;
    ///
    /// let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", 7);
    /// let cur_freq = fs::read_to_string(&path).unwrap();
    /// let cur_freq = cur_freq.parse().unwrap();
    /// let freq_cycles = Cycles::from_khz(cur_freq);
    ///
    /// println!("{}", cycles.as_diff(dur, freq_cycles).unwrap());
    /// ```
    pub fn as_diff(&self, d: Duration, f: Self) -> Result<Self, Box<dyn Error>> {
        let one_secs = Self::from_hz(
            (self.raw * 1_000_000)
                .checked_div(c_ll::try_from(d.as_micros())?)
                .ok_or("Failed to div")?,
        );
        Ok(f - one_secs)
    }
}
