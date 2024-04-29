mod pci_info_enumeration_error;
mod pci_info_error;
mod pci_info_error_string;

use std::{error::Error, fmt::Display};

pub use pci_info_enumeration_error::{
    PciDeviceEnumerationError, PciDeviceEnumerationErrorImpact, PciDeviceEnumerationErrorLocation,
};
pub use pci_info_error::PciInfoError;
pub use pci_info_error_string::PciInfoErrorString;

/// An error returned when trying to access a single property of a PCI device
#[derive(Debug)]
pub enum PciInfoPropertyError {
    /// The required value is not supported by the current enumerator
    Unsupported,
    /// The enumerator encountered an error when trying to get the value
    /// for this property
    Error(Box<PciInfoError>),
}

impl Display for PciInfoPropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported => write!(f, "unsupported by current enumerator"),
            Self::Error(e) => write!(f, "{e}"),
        }
    }
}

impl Error for PciInfoPropertyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Unsupported => None,
            Self::Error(e) => Some(e),
        }
    }
}
