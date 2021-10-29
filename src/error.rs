use std::fmt;
use windows::runtime::HRESULT;

/// A wrapper for winapi errors.
#[derive(Debug)]
pub enum Error {
    RegisterClassFailed,
    Hresult(HRESULT),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RegisterClassFailed => write!(f, "RegisterClass failed"),
            Error::Hresult(hr) => write!(f, "HRESULT 0x{:x}", hr.0),
        }
    }
}

impl std::error::Error for Error {}
