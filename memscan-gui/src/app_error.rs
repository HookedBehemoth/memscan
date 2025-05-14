use std::{
    array::TryFromSliceError, fmt::Display, num::{ParseFloatError, ParseIntError}
};

use nix::errno::Errno;

pub enum AppError {
    PermissionDenied,
    ProcessNotFound,
    DataTypeParseError(String),
    Errno(Errno),
}

impl AppError {
    pub fn from_errno() -> Self {
        let errno = Errno::last();
        match errno {
            nix::errno::Errno::EPERM => AppError::PermissionDenied,
            nix::errno::Errno::ESRCH => AppError::ProcessNotFound,
            _ => AppError::Errno(errno),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::DataTypeParseError(err.to_string())
    }
}

impl From<ParseFloatError> for AppError {
    fn from(err: ParseFloatError) -> Self {
        AppError::DataTypeParseError(err.to_string())
    }
}

impl From<TryFromSliceError> for AppError {
    fn from(err: TryFromSliceError) -> Self {
        AppError::DataTypeParseError(err.to_string())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::PermissionDenied => write!(f, "Permission Denied"),
            AppError::ProcessNotFound => write!(f, "Process Not Found"),
            AppError::DataTypeParseError(input) => {
                write!(f, "{}", input)
            }
            AppError::Errno(errno) => write!(f, "{}", errno),
        }
    }
}
