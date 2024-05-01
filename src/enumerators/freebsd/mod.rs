#[cfg(target_os = "freebsd")]
mod pcidev;

use crate::{PciEnumerator, PciInfo, PciInfoError};

/// A PCI Enumerator for FreeBSD that uses `ioctl(..., PCIOCGETCONF, ...)`
/// operations over `/dev/pci` to extract partial PCI information
pub struct FreeBsdDevPciEnumerator;

impl PciEnumerator for FreeBsdDevPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        unsafe { pcidev::enumerate_devices() }
    }
}

test_enumerator!(FreeBsdDevPciEnumerator, FreeBsdDevPciEnumerator);
