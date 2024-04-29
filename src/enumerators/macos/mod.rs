use crate::{PciEnumerator, PciInfo, PciInfoError};

#[cfg(target_os = "macos")]
mod iokit;

/// A PCI Enumerator for macOS that uses
/// [IOKit](https://developer.apple.com/documentation/iokit) to
/// enumerate devices.
pub struct MacOsIoKitPciEnumerator;

impl PciEnumerator for MacOsIoKitPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        #[cfg(target_os = "macos")]
        iokit::enumerate_pci()
    }
}

test_enumerator!(MacOsIoKitPciEnumerator, MacOsIoKitPciEnumerator);
