use std::{error::Error, fmt::Display};

use super::PciInfoErrorString;

/// Error type for errors encountered during device enumeration.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PciInfoError {
    /// `PciInfo::enumerate_pci` or `default_pci_enumerator()` have
    /// been called on a platform that does not have a
    /// proper implementation for a default enumerator (yet).
    NoDefaultPciEnumeratorForPlatform,

    /// The enumeration has been interrupted by an otherwise non
    /// specified error.
    EnumerationInterrupted(PciInfoErrorString),

    /// An error occurred parsing data. For example it may trigger if
    /// an invalid string like "hello" was found where an hexadecimal
    /// number was expected. The [`PciInfoErrorString`] contained in the
    /// variant carries a brief description of what parsing failed.
    ParseError(PciInfoErrorString),

    /// An I/O error occurred. For example it may trigger if
    /// a file could not be opened because of permissions.
    /// The [`std::io::ErrorKind`] contained withing provides details
    /// about the error.
    IoError(Box<std::io::ErrorKind>),

    /// The end of a file or other resource was reached, when more
    /// data was needed.
    UnexpectedEof,

    /// A value that was expected to be found was, in fact, not found.
    /// For example this could happen on a missing property on an
    /// object, or a missing column on a tsv file. The argument,
    /// optional, provides insight on what value was expected and
    /// not found.
    ValueNotFound(Option<PciInfoErrorString>),

    /// The enumerator was able to read multiple values for a given
    /// property, and those values where not consistent. The slice
    /// contained in the error provides the list of conflicting values
    /// that have been read.
    InconsistentValue(Box<[String]>),

    /// A WMI operation failed. The argument, provides a description
    /// of the failure.
    #[cfg(any(doc, target_os = "windows"))]
    WMIError(PciInfoErrorString),

    /// A Windows operation failed. The causing
    /// [`windows::core::Error`](https://microsoft.github.io/windows-docs-rs/doc/windows/core/struct.Error.html)
    /// is contained within.
    #[cfg(any(doc, target_os = "windows"))]
    Win32Error(windows::core::Error),

    /// A macOS IOKit operation failed. The return value of the
    /// failed API call is contained within (see Apple documentation
    /// for `kern_return_t`).
    #[cfg(any(doc, target_os = "macos"))]
    IoKitError(i32),

    /// A bus-device-function triplet has components that are out of
    /// range. Specifically, while the device should range between
    /// 0 and 31 (5 bits), and function should range between 0 and 7
    /// (3 bits). Reported within are the values of the triplet that
    /// caused the error.
    BdfLocationOutOfRange(u8, u8, u8),

    /// Parsing of an unknown PCI specialized header type has been
    /// attempted.
    UnknownPciHeaderType(u8),

    /// The enumeration has been retried because the device list changed
    /// and the maximum number of iterations has been exceeded.
    DevicesChangedTooManyTimes,
}

impl Display for PciInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PciInfoError::*;

        match self {
            ParseError(s) => write!(f, "error while parsing value: {s}"),
            IoError(e) => write!(f, "i/o error: {e}"),
            UnexpectedEof => write!(f, "unexpected eof while reading data"),
            ValueNotFound(None) => write!(f, "the expected value was not found"),
            ValueNotFound(Some(p)) => write!(f, "the expected value for '{p}' was not found"),
            InconsistentValue(e) => write!(
                f,
                "an inconsistent value was read; considered values were: {e:?}"
            ),
            #[cfg(target_os = "windows")]
            WMIError(m) => write!(f, "WMI error: {m}"),
            #[cfg(target_os = "windows")]
            Win32Error(e) => write!(f, "windows error 0x{:X} ({})", e.code().0, e.message()),
            #[cfg(target_os = "macos")]
            IoKitError(v) => write!(f, "IOKit error {v}"),
            BdfLocationOutOfRange(bus, dev, func) => write!(
                f,
                "the PCI location ({bus:02X}:{dev:02X}.{func:02X} has out of range components"
            ),
            NoDefaultPciEnumeratorForPlatform => {
                write!(f, "this platform does not support a default PCI enumerator")
            }
            UnknownPciHeaderType(h) => write!(f, "unknown PCI header type 0x{h:02X}"),
            EnumerationInterrupted(e) => write!(f, "the enumeration has been interrupted: {e}"),
            DevicesChangedTooManyTimes => {
                write!(f, "the list of PCI devices changed too many times")
            }
        }
    }
}

impl Error for PciInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            #[cfg(target_os = "windows")]
            Self::Win32Error(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PciInfoError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(Box::new(err.kind()))
    }
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for PciInfoError {
    fn from(err: windows::core::Error) -> Self {
        Self::Win32Error(err)
    }
}

#[cfg(target_os = "windows")]
impl From<wmi::WMIError> for PciInfoError {
    fn from(err: wmi::WMIError) -> Self {
        Self::WMIError(format!("{}", err).into())
    }
}

#[cfg(all(doc, not(target_os = "windows")))]
mod windows {
    pub mod core {
        pub struct Error;
    }
}
