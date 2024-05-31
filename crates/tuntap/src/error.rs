use std::{ffi, io, num};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid configuration")]
    InvalidConfig,

    #[error("not implementated")]
    NotImplemented,

    #[error("device name too long")]
    NameTooLong,

    #[error("invalid device name")]
    InvalidName,

    #[error("invalid address")]
    InvalidAddress,

    #[error("invalid file descriptor")]
    InvalidDescriptor,

    #[error("unsuported network layer of operation")]
    UnsupportedLayer,

    #[error("invalid queues number")]
    InvalidQueuesNumber,

    #[cfg(windows)]
    #[error("windows")]
    FakeRegister,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Nul(#[from] ffi::NulError),

    #[error(transparent)]
    ParseNum(#[from] num::ParseIntError),
}

pub type Result<T> = ::std::result::Result<T, Error>;
