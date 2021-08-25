// SPDX-License-Identifier: Apache-2.0

/// Types of potential errors returned by the OS when issuing ioctls to the SEV platform.
use std::fmt::Debug;
use std::{error, io};

/// There are a number of error conditions that can occur between this
/// layer all the way down to the SEV platform. Most of these cases have
/// been enumerated; however, there is a possibility that some error
/// conditions are not encapsulated here.
#[derive(Debug)]
pub enum Indeterminate<T: Debug> {
    /// The error condition is known.
    Known(T),

    /// The error condition is unknown.
    Unknown,
}

/// Error conditions returned by the SEV platform or by layers above it
/// (i.e., the Linux kernel).
///
/// These error conditions are documented in the AMD SEV API spec, but
/// their documentation has been copied here for completeness.
#[derive(Debug)]
pub enum Error {
    /// Something went wrong when communicating with the "outside world"
    /// (kernel, SEV platform).
    IoError(io::Error),

    /// The platform state is invalid for this command.
    InvalidPlatformState,

    /// The guest state is invalid for this command.
    InvalidGuestState,

    /// The platform configuration is invalid.
    InvalidConfig,

    /// A memory buffer is too small.
    InvalidLen,

    /// The platform is already owned.
    AlreadyOwned,

    /// The certificate is invalid.
    InvalidCertificate,

    /// Request is not allowed by guest policy.
    PolicyFailure,

    /// The guest is inactive.
    Inactive,

    /// The address provided is invalid.
    InvalidAddress,

    /// The provided signature is invalid.
    BadSignature,

    /// The provided measurement is invalid.
    BadMeasurement,

    /// The ASID is already owned.
    AsidOwned,

    /// The ASID is invalid.
    InvalidAsid,

    /// WBINVD instruction required.
    WbinvdRequired,

    /// `DF_FLUSH` invocation required.
    DfFlushRequired,

    /// The guest handle is invalid.
    InvalidGuest,

    /// The command issued is invalid.
    InvalidCommand,

    /// The guest is active.
    Active,

    /// A hardware condition has occurred affecting the platform. It is safe
    /// to re-allocate parameter buffers.
    HardwarePlatform,

    /// A hardware condition has occurred affecting the platform. Re-allocating
    /// parameter buffers is not safe.
    HardwareUnsafe,

    /// Feature is unsupported.
    Unsupported,

    /// A given parameter is invalid.
    InvalidParam,

    /// The SEV firmware has run out of a resource required to carry out the
    /// command.
    ResourceLimit,

    /// The SEV platform observed a failed integrity check.
    SecureDataInvalid,
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        match self {
            Self::IoError(_) => "I/O Error",
            Self::InvalidPlatformState => "Invalid platform state",
            Self::InvalidGuestState => "Invalid guest state",
            Self::InvalidConfig => "Platform configuration invalid",
            Self::InvalidLen => "Memory buffer too small",
            Self::AlreadyOwned => "Platform is already owned",
            Self::InvalidCertificate => "Invalid certificate",
            Self::PolicyFailure => "Policy failure",
            Self::Inactive => "Guest is inactive",
            Self::InvalidAddress => "Provided address is invalid",
            Self::BadSignature => "Provided signature is invalid",
            Self::BadMeasurement => "Provided measurement is invalid",
            Self::AsidOwned => "ASID is already owned",
            Self::InvalidAsid => "ASID is invalid",
            Self::WbinvdRequired => "WBINVD instruction required",
            Self::DfFlushRequired => "DF_FLUSH invocation required",
            Self::InvalidGuest => "Guest handle is invalid",
            Self::InvalidCommand => "Issued command is invalid",
            Self::Active => "Guest is active",
            Self::HardwarePlatform => "HW condition, safe to re-allocate parameter buffers",
            Self::HardwareUnsafe => "HW condition, unsafe to re-allocate parameter buffers",
            Self::Unsupported => "Feature is unsupported",
            Self::InvalidParam => "Given parameter is invalid",
            Self::ResourceLimit => "SEV firmware is out of required resources",
            Self::SecureDataInvalid => "SEV platform observed a failed integrity check",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<io::Error> for Indeterminate<Error> {
    #[inline]
    fn from(error: io::Error) -> Indeterminate<Error> {
        Indeterminate::Known(error.into())
    }
}

impl From<Indeterminate<Error>> for io::Error {
    #[inline]
    fn from(indeterminate: Indeterminate<Error>) -> io::Error {
        match indeterminate {
            Indeterminate::Known(e) => io::Error::new(io::ErrorKind::Other, e),
            Indeterminate::Unknown => io::Error::new(io::ErrorKind::Other, "unknown SEV error"),
        }
    }
}

impl From<u32> for Indeterminate<Error> {
    #[inline]
    fn from(error: u32) -> Indeterminate<Error> {
        Indeterminate::Known(match error {
            0 => io::Error::last_os_error().into(),
            1 => Error::InvalidPlatformState,
            2 => Error::InvalidGuestState,
            3 => Error::InvalidConfig,
            4 => Error::InvalidLen,
            5 => Error::AlreadyOwned,
            6 => Error::InvalidCertificate,
            7 => Error::PolicyFailure,
            8 => Error::Inactive,
            9 => Error::InvalidAddress,
            10 => Error::BadSignature,
            11 => Error::BadMeasurement,
            12 => Error::AsidOwned,
            13 => Error::InvalidAsid,
            14 => Error::WbinvdRequired,
            15 => Error::DfFlushRequired,
            16 => Error::InvalidGuest,
            17 => Error::InvalidCommand,
            18 => Error::Active,
            19 => Error::HardwarePlatform,
            20 => Error::HardwareUnsafe,
            21 => Error::Unsupported,
            22 => Error::InvalidParam,
            23 => Error::ResourceLimit,
            24 => Error::SecureDataInvalid,
            _ => return Indeterminate::Unknown,
        })
    }
}
