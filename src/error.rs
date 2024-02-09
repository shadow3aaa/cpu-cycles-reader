use std::{num::TryFromIntError, result};

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("TryFromIntError")]
    TryFromIntError(#[from] TryFromIntError),
    #[error("Failed to create cycles reader")]
    FailedToCreate,
    #[error("Failed to read cpu cycles")]
    FailedToRead,
    #[error("Cpu cores of CyclesInstant are inconsistent and cannot be subtracted")]
    InconsistentCore,
}
