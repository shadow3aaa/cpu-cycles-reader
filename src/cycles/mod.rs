//! Cycles provides a way to represent the number of cycles

mod ext;
mod trans;

use std::fmt::{self, Display, Formatter};

use derive_more::{Add, Constructor, Div, From, Into, Mul, Sub, Sum};
use libc::c_longlong as c_ll;

pub use ext::*;
pub use trans::*;

/// Cycles provides a way to represent the number of cycles
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
