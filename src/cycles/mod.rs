//! Cycles provides a way to represent the number of cycles

mod trans;

use std::fmt::{self, Display, Formatter};

use derive_more::{
    Add, AddAssign, Constructor, Div, DivAssign, From, Into, Mul, MulAssign, Neg, Rem, RemAssign,
    Sub, SubAssign, Sum,
};
use libc::c_longlong as c_ll;

/// Represent the number of cpu cycles
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
    Neg,
    Rem,
    SubAssign,
    AddAssign,
    RemAssign,
    DivAssign,
    MulAssign,
)]
pub struct Cycles {
    raw: c_ll,
}

#[allow(clippy::cast_precision_loss)]
impl Display for Cycles {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.raw >= 1_000_000_000 {
            write!(f, "{:.2}Ghz", self.raw as f64 / 1_000_000_000.0)
        } else if self.raw >= 1_000_000 {
            write!(f, "{:.2}Mhz", self.raw as f64 / 1_000_000.0)
        } else if self.raw >= 1000 {
            write!(f, "{:.2}Khz", self.raw as f64 / 1000.0)
        } else {
            write!(f, "{}Hz", self.raw)
        }
    }
}

impl Default for Cycles {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Cycles {
    pub const MAX: Self = Self { raw: c_ll::MAX };
    pub const ZERO: Self = Self { raw: 0 };

    #[must_use]
    pub fn mul_f64(&self, rhs: f64) -> Self {
        Self::new((self.raw as f64 * rhs) as i64)
    }

    #[must_use]
    pub fn div_f64(&self, rhs: f64) -> Self {
        Self::new((self.raw as f64 / rhs) as i64)
    }

    #[must_use]
    pub fn mul_f32(&self, rhs: f32) -> Self {
        Self::new((self.raw as f32 * rhs) as i64)
    }

    #[must_use]
    pub fn div_f32(&self, rhs: f32) -> Self {
        Self::new((self.raw as f32 / rhs) as i64)
    }
}
