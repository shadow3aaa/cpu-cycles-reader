//! Expansion of Cycles struct

use std::{error::Error, fs, time::Duration};

use libc::{c_int, c_longlong as c_ll};

use super::Cycles;

impl Cycles {
    /// Returns the average number of cpu usage within a specified time
    /// May be >100%, because the current frequency read is only a suggested frequency, not a real frequency, but the cycles are real
    /// d: record the time of cycles
    /// c: cpu core number
    /// Read `/sys/devices/system/cpu/cpuX/cpufreq/scaling_cur_freq` to get the current frequency
    /// ```ignore
    /// use std::time::{Duration, Instant};
    /// use cpu_cycles_reader::Cycles;
    ///
    /// let now = Instant::now();
    /// let cycles_former = Cycles::from_ghz(1);
    ///
    /// // cpu进行了一些操作，假设我们记录的是cpu7
    ///
    /// let dur = Instant::now() - now;
    /// let cycles_later = Cycles::from_ghz(2);
    ///
    /// let cycles = cycles_later - cycles_former;
    /// println!("{:.2}", cycles.as_usage(dur, 7).unwrap());
    /// ```
    pub fn as_usage(&self, d: Duration, c: u64) -> Result<f64, Box<dyn Error>> {
        let hz = (self.raw as u128 * 1_000_000)
            .checked_div(d.as_micros())
            .ok_or("Failed to div")?;

        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", c);
        let cur_freq = fs::read_to_string(path)?;
        let cur_hz: u128 = cur_freq.trim().parse::<u128>()? * 1000;

        Ok(hz as f64 / cur_hz as f64)
    }

    /// Similar to as_usage, but returns the difference from the current frequency [`self::Cycles`]
    /// For the same reason, diff may be negative
    /// d: record the time of cycles
    /// c: cpu core number
    /// 读取`/sys/devices/system/cpu/cpuX/cpufreq/scaling_cur_freq`来获取当前频率
    /// ```ignore
    /// use std::time::{Duration, Instant};
    /// use cpu_cycles_reader::Cycles;
    ///
    /// let now = Instant::now();
    /// let cycles_former = Cycles::from_ghz(1);
    ///
    /// // The cpu has performed some operations, assuming we are recording the cpu7
    ///
    /// let dur = Instant::now() - now;
    /// let cycles_later = Cycles::from_ghz(2);
    ///
    /// let cycles = cycles_later - cycles_former;
    /// println!("{}", cycles.as_diff(dur, 7).unwrap());
    /// ```
    pub fn as_diff(&self, d: Duration, c: c_int) -> Option<Cycles> {
        let hz = Cycles::from_hz((self.raw * 1_000_000).checked_div(d.as_micros() as c_ll)?);

        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", c);
        let cur_freq = fs::read_to_string(path).ok()?;
        let cur_hz = Cycles::from_khz(cur_freq.trim().parse::<c_ll>().ok()?);

        Some(cur_hz - hz)
    }
}
