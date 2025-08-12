use crate::PciBusNumber;
use crate::PciInfoError;
use crate::PciInfoErrorString;
use crate::PciLocation;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

/// The location at which an enumeration error occurred
#[derive(Copy, Clone, Debug)]
pub enum PciDeviceEnumerationErrorLocation {
    /// The PCI location where the error occurred is unknown
    None,
    /// The error occurred at the specified bus
    Bus(PciBusNumber),
    /// The error occurred at the specified PCI location
    Device(PciLocation),
}

/// The impact of an enumeration error
#[derive(Copy, Clone, Debug)]
pub enum PciDeviceEnumerationErrorImpact {
    /// The error might have caused an entire PCI bus to be missing
    /// from the enumeration
    Bus,
    /// The error might have caused an entire PCI device to be missing
    /// from the enumeration
    Device,
    /// The error might have caused some properties on a PCI device to be
    /// missing from the enumeration. When these are generated it is
    /// usually not possible to backtrack to which device caused the
    /// properties to be missing. Missing properties will be reported as if
    /// unsupported by the enumerator.
    DeviceProperties,
}

/// A non-fatal error that impacted the enumeration of one or more devices.
#[derive(Debug)]
pub struct PciDeviceEnumerationError {
    impact: PciDeviceEnumerationErrorImpact,
    error: PciInfoError,
    location: PciDeviceEnumerationErrorLocation,
}

impl Display for PciDeviceEnumerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let at: PciInfoErrorString = match self.location {
            PciDeviceEnumerationErrorLocation::None => "".into(),
            PciDeviceEnumerationErrorLocation::Bus(l) => format!(" at {l:?} ").into(),
            PciDeviceEnumerationErrorLocation::Device(l) => format!(" at {l:?} ").into(),
        };

        match self.impact {
            PciDeviceEnumerationErrorImpact::Bus => {
                write!(f, "pci bus enumeration error{}: {}", at, self.error)
            }
            PciDeviceEnumerationErrorImpact::Device => {
                write!(f, "pci device enumeration error{}: {}", at, self.error)
            }
            PciDeviceEnumerationErrorImpact::DeviceProperties => write!(
                f,
                "an error might have caused properties to be missing from a pci device{}: {}",
                at, self.error
            ),
        }
    }
}

impl PciDeviceEnumerationError {
    #[allow(dead_code)]
    pub(crate) fn new(impact: PciDeviceEnumerationErrorImpact, error: PciInfoError) -> Self {
        Self {
            impact,
            error,
            location: PciDeviceEnumerationErrorLocation::None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_at_bus(
        bus: PciBusNumber,
        impact: PciDeviceEnumerationErrorImpact,
        error: PciInfoError,
    ) -> Self {
        Self {
            impact,
            error,
            location: PciDeviceEnumerationErrorLocation::Bus(bus),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_at_device(
        loc: PciLocation,
        impact: PciDeviceEnumerationErrorImpact,
        error: PciInfoError,
    ) -> Self {
        Self {
            impact,
            error,
            location: PciDeviceEnumerationErrorLocation::Device(loc),
        }
    }

    /// The impact of this error
    pub fn impact(&self) -> PciDeviceEnumerationErrorImpact {
        self.impact
    }

    /// The underlying error that caused this issue (same as
    /// `Error::source()` but more strongly typed)
    pub fn error(&self) -> &PciInfoError {
        &self.error
    }

    /// The location where the issue occurred, if available.
    /// Note that errors often occur before a location can be
    /// determined.
    pub fn location(&self) -> PciDeviceEnumerationErrorLocation {
        self.location
    }
}

impl Error for PciDeviceEnumerationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.error())
    }
}
