use derive_more::{Add, Constructor, Div, From, Into, Mul, Sub, Sum};
use libc::c_longlong as c_ll;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs,
    time::Duration,
};

#[derive(
    Clone,
    Copy,
    PartialEq,
    Add,
    Sub,
    From,
    Into,
    Mul,
    Div,
    Sum,
    Constructor,
    Debug,
    PartialOrd,
    Eq,
    Ord,
)]
pub struct Cycles {
    raw: c_ll,
}

impl Display for Cycles {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.raw > 1_000_000_000 {
            write!(f, "{:.2}Ghz", self.raw as f64 / 1_000_000_000.0)
        } else if self.raw > 1_000_000 {
            write!(f, "{:.2}Mhz", self.raw as f64 / 1_000_000.0)
        } else if self.raw > 1000 {
            write!(f, "{:.2}Khz", self.raw as f64 / 1000.0)
        } else {
            write!(f, "{}Hz", self.raw)
        }
    }
}

impl Cycles {
    /// 从hz构造
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1000), Cycles::from_hz(1000));
    /// ```
    pub fn from_hz(h: c_ll) -> Self {
        Self { raw: h }
    }

    /// 从khz构造
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1000), Cycles::from_khz(1));
    /// ```
    pub fn from_khz(k: c_ll) -> Self {
        Self { raw: k * 1000 }
    }

    /// 从mhz构造
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(2_000_000), Cycles::from_mhz(2));
    /// ```
    pub fn from_mhz(m: c_ll) -> Self {
        Self { raw: m * 1_000_000 }
    }

    /// 从ghz构造
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(3_000_000_000), Cycles::from_ghz(3));
    /// ```
    pub fn from_ghz(g: c_ll) -> Self {
        Self {
            raw: g * 1_000_000_000,
        }
    }

    /// 以hz为单位的整数
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1000).as_hz(), 1000);
    /// ```
    pub fn as_hz(&self) -> c_ll {
        self.raw
    }

    /// 以Khz为单位的整数
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1_000_000).as_khz(), 1000);
    /// ```
    pub fn as_khz(&self) -> c_ll {
        self.raw / 1000
    }

    /// 以Mhz为单位的整数
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1_000_000_000).as_mhz(), 1000);
    /// ```
    pub fn as_mhz(&self) -> c_ll {
        self.raw / 1_000_000
    }

    /// 以Ghz为单位的整数
    /// ```
    /// use cpu_cycles_reader::Cycles;
    ///
    /// assert_eq!(Cycles::new(1_000_000_000_000).as_ghz(), 1000);
    /// ```
    pub fn as_ghz(&self) -> c_ll {
        self.raw / 1_000_000_000
    }

    /// 返回指定时间内的cpu使用率平均数
    /// 可能＞100％，因为读取的当前频率只是建议频率，而不是真实频率，但是cycles是真实的
    /// d: 记录cycles的时间
    /// c: cpu核心编号
    /// 读取`/sys/devices/system/cpu/cpuX/cpufreq/scaling_cur_freq`来获取当前频率
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

    /// 类似as_usage，但是返回和当前的频率的差值[`self::Cycles`]
    /// 同样的理由，diff可能会是负数
    /// d: 记录cycles的时间
    /// c: cpu核心编号
    /// 读取`/sys/devices/system/cpu/cpuX/cpufreq/scaling_cur_freq`来获取当前频率
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
    /// println!("{}", cycles.as_diff(dur, 7).unwrap());
    /// ```
    pub fn as_diff(&self, d: Duration, c: u64) -> Option<Cycles> {
        let hz = Cycles::from_hz((self.raw * 1_000_000).checked_div(d.as_micros() as c_ll)?);

        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", c);
        let cur_freq = fs::read_to_string(path).ok()?;
        let cur_hz = Cycles::from_khz(cur_freq.trim().parse::<c_ll>().ok()?);

        Some(cur_hz - hz)
    }
}
