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
/// to enumerate devices. Requires the `enum_win32_wmi` feature to be enabled.
#[cfg(any(feature = "enum_win32_wmi", doc))]
pub struct WindowsWmiPciEnumerator;
#[cfg(any(feature = "enum_win32_wmi", doc))]
impl PciEnumerator for WindowsWmiPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        #[cfg(target_os = "windows")]
        wmi::enumerate_pci()
    }
}

#[cfg(feature = "enum_win32_wmi")]
test_enumerator!(WindowsWmiPciEnumerator, WindowsWmiPciEnumerator);
test_enumerator!(WindowsSetupApiPciEnumerator, WindowsSetupApiPciEnumerator);
