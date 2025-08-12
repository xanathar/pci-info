// Dead code is allowed in this module as it serves multiple platforms
#![allow(dead_code)]

use crate::{enumerators, PciDevice, PciDeviceEnumerationError, PciEnumerator, PciInfoError};

/// Holds the result of an enumeration of PCI devices.
/// Use the `enumerate_pci` and `enumerate_pci_with_enumerator` methods
/// to enumerate PCI devices and create a new instance, or simply use
/// the `PciEnumerator::enumerate_pci` trait method.
///
/// The `PciInfo` can be iterated on, either directly or calling the
/// `iter()` or `into_iter()` methods.
///
/// # Example
/// ```rust
/// use pci_info::PciInfo;
///
/// // Enumerate the devices on the PCI bus using the default
/// // enumerator for the current platform. The `unwrap()` panics if
/// // the enumeration fatally fails.
/// let info = PciInfo::enumerate_pci().unwrap();
///
/// // Print out some properties of the enumerated devices.
/// // Note that the collection contains both devices and errors
/// // as the enumeration of PCI devices can fail entirely (in which
/// // case `PciInfo::enumerate_pci()` would return error) or
/// // partially (in which case an error would be inserted in the
/// // result).
/// for r in info {
///     match r {
///         Ok(device) => println!("{device:?}"),
///         Err(error) => eprintln!("{error:?}"),
///     }
/// }
/// ```
///
pub struct PciInfo {
    pub(crate) results: Vec<Result<PciDevice, PciDeviceEnumerationError>>,
}

impl PciInfo {
    pub(crate) fn empty() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub(crate) fn push_device(&mut self, dev: PciDevice) {
        self.results.push(Ok(dev));
    }

    pub(crate) fn push_error(&mut self, err: PciDeviceEnumerationError) {
        self.results.push(Err(err));
    }

    /// Creates a new `PciInfo` using the default `PciEnumerator`.
    pub fn enumerate_pci() -> Result<Self, PciInfoError> {
        Self::enumerate_pci_with_enumerator(enumerators::default_pci_enumerator()?)
    }

    /// Creates a new `PciInfo` using the specified `PciEnumerator`.
    pub fn enumerate_pci_with_enumerator<T: PciEnumerator>(
        enumerator: T,
    ) -> Result<Self, PciInfoError> {
        enumerator.enumerate_pci()
    }

    /// Returns an iterator to the results references
    pub fn iter(&self) -> PciInfoRefIterator<'_> {
        PciInfoRefIterator {
            pci_info: self,
            index: 0,
        }
    }

    pub(crate) fn find_device_mut(
        &mut self,
        location: crate::PciLocation,
    ) -> Option<&mut PciDevice> {
        for r in self.results.iter_mut().flatten() {
            if let Some(loc) = r.properties.location.as_option() {
                if *loc == location {
                    return Some(r);
                }
            }
        }
        None
    }

    pub(crate) fn mutate_devices<F: FnMut(&mut PciDevice)>(&mut self, mut mutator: F) {
        for r in self.results.iter_mut().flatten() {
            mutator(r);
        }
    }
}

impl<'a> IntoIterator for &'a PciInfo {
    type Item = Result<&'a PciDevice, &'a PciDeviceEnumerationError>;
    type IntoIter = PciInfoRefIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for PciInfo {
    type Item = Result<PciDevice, PciDeviceEnumerationError>;
    type IntoIter = <Vec<Result<PciDevice, PciDeviceEnumerationError>> as IntoIterator>::IntoIter;

    /// Returns an iterator to the results, consuming the `PciInfo` in
    /// the process
    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

pub struct PciInfoRefIterator<'a> {
    pci_info: &'a PciInfo,
    index: usize,
}

impl<'a> Iterator for PciInfoRefIterator<'a> {
    type Item = Result<&'a PciDevice, &'a PciDeviceEnumerationError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.pci_info.results.len() {
            None
        } else {
            let item = match &self.pci_info.results[self.index] {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            };
            self.index += 1;
            Some(item)
        }
    }
}
