//! This module implements the various enumerations that can be used to enumerate
//! PCI devices. In addition, it also provides the `default_pci_enumerator()`
//! function to create the current platform's default enumerator.
//!
//! # Currently implemented enumerators
//!
//! Enumerator                                     | Platforms | PCI id | PCI location | Revision | Device class | PCI subsystem | Assigned IRQ | OS driver
//! ---------------------------------------------- | --------- | ------ | ------------ | -------- | ------------ | ----------------- | ------------ | ----------
//! [`FreeBsdDevPciEnumerator`]                        | FreeBSD | ✅ | ✅             | ✅ | ✅ | ✅ | ❌ | ✅
//! [`LinuxProcFsPciEnumerator::Fastest`]              | Linux   | ✅ | ✅<sup>2</sup> | ❌ | ❌ | ❌ | ✅ | ✅
//! [`LinuxProcFsPciEnumerator::HeadersOnly`]          | Linux   | ✅ | ✅<sup>2</sup> | ✅ | ✅ | ✅ | ❌ | ❌
//! [`LinuxProcFsPciEnumerator::SkipNoncommonHeaders`] | Linux   | ✅ | ✅<sup>2</sup> | ✅ | ✅ | ❌ | ✅ | ✅
//! [`LinuxProcFsPciEnumerator::Exhaustive`]           | Linux   | ✅ | ✅<sup>2</sup> | ✅ | ✅ | ✅ | ✅ | ✅
//! [`MacOsIoKitPciEnumerator`]<sup>3</sup>            | macOS   | ✅ | ⚠️<sup>1, 2</sup> | ✅ | ✅ | ✅ | ❌ | ❌
//! [`WindowsSetupApiPciEnumerator`]                   | Windows | ✅ | ⚠️<sup>1, 2</sup> | ✅ | ✅ | ✅ | ❌ | ❌
//! [`WindowsWmiPciEnumerator`]                        | Windows | ✅ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌
//!
//! Notes:
//! - (1) = The PCI location on this enumerator is parsed from human readable strings; that parsing might fail or the information might be incorrect.
//! - (2) = The PCI location on this enumerator might not support multiple PCI segments/domains correctly.
//! - (3) = Apparently most of the devices in Apple silicon Macs are not PCI/PCIe. As such PCI enumeration on Apple silicon computers return quite a short list.

use crate::{pci_info::PciInfo, PciInfoError};

#[cfg(any(doc, target_os = "windows"))]
mod windows;
#[cfg(any(doc, target_os = "windows"))]
pub use windows::*;

#[cfg(any(doc, target_os = "macos"))]
mod macos;
#[cfg(any(doc, target_os = "macos"))]
pub use macos::*;

#[cfg(any(doc, target_os = "linux"))]
mod linux;
#[cfg(any(doc, target_os = "linux"))]
pub use linux::*;

#[cfg(any(doc, target_os = "freebsd"))]
mod freebsd;
#[cfg(any(doc, target_os = "freebsd"))]
pub use freebsd::*;

/// A trait that is implemented by all types able to enumerate PCI
/// devices.
pub trait PciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError>;
}

/// Creates the default PCI enumerator for the platform in use. If
/// no default PCI enumerator is available for the platform, it
/// returns `PciInfoError::NoDefaultPciEnumeratorForPlatform`.
pub fn default_pci_enumerator() -> Result<impl PciEnumerator, PciInfoError> {
    #[cfg(target_os = "windows")]
    return Ok(WindowsSetupApiPciEnumerator);

    #[cfg(target_os = "macos")]
    return Ok(MacOsIoKitPciEnumerator);

    #[cfg(target_os = "freebsd")]
    return Ok(FreeBsdDevPciEnumerator);

    #[cfg(target_os = "linux")]
    return Ok(LinuxProcFsPciEnumerator::Exhaustive);

    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "windows",
        target_os = "freebsd"
    )))]
    Err::<InvalidPciEnumerator, PciInfoError>(PciInfoError::NoDefaultPciEnumeratorForPlatform)
}

#[allow(dead_code)]
struct InvalidPciEnumerator;
impl PciEnumerator for InvalidPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        unreachable!()
    }
}
