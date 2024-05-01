#![allow(clippy::needless_doctest_main)]
//! The `pci-info` crate provides a simple API to enumerate PCI devices across "desktop" operating systems (Linux, Windows, MacOS, FreeBSD, with more to be added), or to parse PCI headers from files or memory buffers.
//!
//! It supports parsing of PCI metadata and availability of the various fields (including possibly the entire standard PCI configuration space of a device); the level of support is subject to the capabilities of the enumerator in use.
//!
//! It uses user-mode APIs only, accessible from normal users (i.e. no root/Administrator needed). All code has been implemented from scratch through openly available documentation. As such it usually provides less data than some alternative solutions (e.g. libpci) but with less strict requirements (both in terms of licensing and available properties on some platforms).
//!
//! PCI device classes, subclassses and interface functions are optionally exposed as rusty enums for quick matching, and have been implemented from publicly available documentation.
//!
//! PCI vendor and device ids are kept as `u16` and have to be manually interpreted or transformed into strings using other crates; the publicly available list of PCI vendors and devices is intentionally not included to contain the crate size (the complete list is large) and to avoid licensing issues (this crate being MIT+Apache dual licensed, the list being GPL+BSD dual licensed).
//!
//! # Functionality
//! - Enumeration of devices using OS usermode APIs on Windows, Linux and macOS, with more platforms to be added. See the [`PciInfo`] type.
//! - Parsing of PCI headers starting from byte arrays of the PCI configuration space. See the [`pci_headers`] module.
//! - Parsing of PCI device classes, subclasses and interface-functions from their codes. See the [`pci_enums`] module.
//!
//!
//! # Examples
//!
//! Using the library with a default enumerator is straightforward:
//!
//! ```rust
//! use pci_info::PciInfo;
//!
//! pub fn main() {
//!     // Enumerate the devices on the PCI bus using the default
//!     // enumerator for the current platform. The `unwrap()` panics if
//!     // the enumeration fatally fails.
//!     let info = PciInfo::enumerate_pci().unwrap();
//!
//!     // Print out some properties of the enumerated devices.
//!     // Note that the collection contains both devices and errors
//!     // as the enumeration of PCI devices can fail entirely (in which
//!     // case `PciInfo::enumerate_pci()` would return error) or
//!     // partially (in which case an error would be inserted in the
//!     // result).
//!     for r in info {
//!         match r {
//!             Ok(device) => println!("{device:?}"),
//!             Err(error) => eprintln!("{error}"),
//!         }
//!     }
//! }
//! ```
//!
//! ## Using a custom enumerator
//!
//! If so desired, a custom enumerator can be used, using the
//! `enumerate_pci_with_enumerator` method of the `PciInfo` type.
//!
//! For example:
//!
//! ```rust
//! // NOTE: This example runs only on Windows as it uses a platform
//! // specific PCI enumerator.
//!
//! # #[cfg(target_os = "windows")]
//! use pci_info::{enumerators, PciInfo, PciInfoError};
//!
//! #[cfg(target_os = "windows")]
//! pub fn main() -> Result<(), PciInfoError> {
//!     // Enumerate the devices by accessing the `enumerate_pci_with_enumerator`
//!     // method of the `PciInfo` type.
//!     let info = PciInfo::enumerate_pci_with_enumerator(enumerators::WindowsWmiPciEnumerator)?;
//!
//!     for r in info {
//!         match r {
//!             Ok(device) => println!("{device:?}"),
//!             Err(error) => eprintln!("{error}"),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! # #[cfg(not(target_os = "windows"))]
//! # pub fn main() {}
//! ```
//!
//! Alternatively, you can directly call the `enumerate_pci` of the
//! `PciEnumerator` trait. The recommended syntax is through the
//! `PciInfo::enumerate_pci_with_enumerator` method, though.
//!
//!
//! ```rust
//! // NOTE: This example runs only on Linux as it uses a platform
//! // specific PCI enumerator.
//!
//! # #[cfg(target_os = "linux")]
//! use pci_info::{enumerators, enumerators::PciEnumerator, PciInfoError};
//!
//! #[cfg(target_os = "linux")]
//! pub fn main() -> Result<(), PciInfoError> {
//!     // Create a Linux-specific custom enumerator.
//!     let enumerator = enumerators::LinuxProcFsPciEnumerator::Fastest;
//!     // Enumerate the devices by accessing the `enumerate_pci`
//!     // method of the `PciEnumerator` trait. Works but using
//!     // `PciInfo::enumerate_pci_with_enumerator` is preferred.
//!     let info = enumerator.enumerate_pci()?;
//!
//!     for r in info {
//!         match r {
//!             Ok(device) => println!("{device:?}"),
//!             Err(error) => eprintln!("{error}"),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! # #[cfg(not(target_os = "linux"))]
//! # pub fn main() {}
//! ```
//!
//! # Features
//!
//! The crate is configurable with the following features:
//!
//! Crate feature | Default | Description
//! ------------- | ------ | ------------
//! `pci_class_debug_strings` | YES | Includes human readable debug strings for variants of [`pci_enums::PciDeviceClass`]. Disable to reduce the binary size.
//! `pci_subclass_debug_strings` | YES | Includes human readable debug strings for variants of [`pci_enums::PciDeviceSubclass`]. Disable to reduce the binary size.
//! `pci_interface_func_debug_string` | YES | Includes human readable debug strings for variants of [`pci_enums::PciDeviceInterfaceFunc`]. Disable to reduce the binary size.
//!

#[macro_use]
mod macros;

mod error;
mod pci_bus_number;
mod pci_device;
mod pci_info;
mod pci_location;
mod pci_property_result;

pub mod enumerators;
pub mod pci_enums;
pub mod pci_headers;

pub use error::{
    PciDeviceEnumerationError, PciDeviceEnumerationErrorImpact, PciDeviceEnumerationErrorLocation,
    PciInfoError, PciInfoErrorString, PciInfoPropertyError,
};
pub use pci_bus_number::PciBusNumber;
pub use pci_device::PciDevice;
pub use pci_info::PciInfo;
pub use pci_location::PciLocation;

pub use enumerators::{default_pci_enumerator, PciEnumerator};
