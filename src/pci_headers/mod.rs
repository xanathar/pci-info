//! This module offers access to the raw values of the PCI
//! configuration space headers, if (and only if) supported by
//! the PCI enumerator in use.
//!
//! Access starts from the [`PciCommonHeader`] type; for more property information
//! create a [`PciSpecializedHeader`] using the [`PciCommonHeader::header_type`]
//! field.
//!
//! # Example
//! ```rust
//! // PCI header of an Intel 82371SB PIIX3 southbridge ISA bridge
//! static PCI_HEADER_BYTES: &[u8] = &[
//!     0x86, 0x80, 0x00, 0x70, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0x00, 0x00, 0x80, 0x00,
//!     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
//! ];
//!
//! use pci_info::{pci_headers::*, PciDevice};
//!
//! pub fn main() {
//!     // Read the PCI common-header
//!     let common_header = PciCommonHeader::with_bytes(&PCI_HEADER_BYTES).unwrap();
//!     // Read the rest of the header
//!     let specialized_header = PciSpecializedHeader::read_subheader(common_header.header_type, &PCI_HEADER_BYTES, true).unwrap();
//!
//!     println!("Common header: {common_header:?}");
//!     println!("Specialized header: {specialized_header:?}");
//!
//!     // We can parse more readable information from the headers
//!     let device = PciDevice::from_pci_header_set(common_header, Some(specialized_header));
//!
//!     // This should print 'Bridge_IsaBridge_Default'
//!     println!("Device i-f: {:?}", device.device_iface().unwrap())
//! }
//! ```

mod pci_common_header;
mod pci_config_buffer;
mod pci_generic_device_header;
mod pci_specialized_header;
mod pci_to_cardbus_bridge_header;
mod pci_to_pci_bridge_header;

pub use pci_common_header::PciCommonHeader;
pub use pci_generic_device_header::PciGenericDeviceHeader;
pub use pci_specialized_header::PciSpecializedHeader;
pub use pci_to_cardbus_bridge_header::PciToCardbusBridgeHeader;
pub use pci_to_pci_bridge_header::PciToPciBridgeHeader;
