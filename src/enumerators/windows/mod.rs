use crate::{PciEnumerator, PciInfo, PciInfoError};

#[cfg(target_os = "windows")]
mod setupapi;
#[cfg(target_os = "windows")]
mod wmi;

#[cfg(target_os = "windows")]
mod common;

/// A PCI Enumerator for Windows that uses
/// [SetupAPI](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/setupapi)
/// to enumerate devices.
pub struct WindowsSetupApiPciEnumerator;
impl PciEnumerator for WindowsSetupApiPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        #[cfg(target_os = "windows")]
        setupapi::enumerate_pci()
    }
}

/// A PCI Enumerator for Windows that uses
/// [WMI](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-start-page)
/// to enumerate devices.
pub struct WindowsWmiPciEnumerator;
impl PciEnumerator for WindowsWmiPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        #[cfg(target_os = "windows")]
        wmi::enumerate_pci()
    }
}

test_enumerator!(WindowsWmiPciEnumerator, WindowsWmiPciEnumerator);
test_enumerator!(WindowsSetupApiPciEnumerator, WindowsSetupApiPciEnumerator);
