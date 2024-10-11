use crate::PciInfoError;

use super::{
    pci_config_buffer::PciConfigBuffer, pci_generic_device_header::PciGenericDeviceHeader,
    pci_to_cardbus_bridge_header::PciToCardbusBridgeHeader,
    pci_to_pci_bridge_header::PciToPciBridgeHeader, PciCommonHeader,
};

/// Enumeration of the supported specialized headers of PCI devices.
#[derive(Clone, Debug)]
pub enum PciSpecializedHeader {
    /// Sub-header for a generic PCI device
    GenericDevice(PciGenericDeviceHeader),
    /// Sub-header for PCI-to-PCI bus bridge
    PciToPciBridge(PciToPciBridgeHeader),
    /// Sub-header for PCI-to-Cardbus bus bridge
    PciToCardbusBridge(PciToCardbusBridgeHeader),
}

impl PciSpecializedHeader {
    /// Returns the length of the "sub-header" for the specified
    /// `header_type`.
    pub fn length_of_subheader(header_type: u8) -> Option<usize> {
        match header_type & 0x7F {
            PciGenericDeviceHeader::ID => Some(PciGenericDeviceHeader::LENGTH),
            PciToPciBridgeHeader::ID => Some(PciToPciBridgeHeader::LENGTH),
            PciToCardbusBridgeHeader::ID => Some(PciToCardbusBridgeHeader::LENGTH),
            _ => None,
        }
    }

    /// Reads the data of the sub-header from a memory buffer.
    /// - `bytes` : specifies the slice containing the data to be read
    /// - `includes_common_header` : true if the slice starts at the beginning
    ///   of the common header, false if the slice contains only the sub-header data
    pub fn read_subheader(
        header_type: u8,
        bytes: &[u8],
        includes_common_header: bool,
    ) -> Result<Self, PciInfoError> {
        let bytes_offset = if includes_common_header {
            0
        } else {
            PciCommonHeader::COMMON_HEADER_LEN
        };

        let pci_cfg = PciConfigBuffer::new(bytes, bytes_offset);

        match header_type & 0x7F {
            PciGenericDeviceHeader::ID => Ok(PciSpecializedHeader::GenericDevice(
                PciGenericDeviceHeader::with_pci_cfg(&pci_cfg)?,
            )),
            PciToPciBridgeHeader::ID => Ok(PciSpecializedHeader::PciToPciBridge(
                PciToPciBridgeHeader::with_pci_cfg(&pci_cfg)?,
            )),
            PciToCardbusBridgeHeader::ID => Ok(PciSpecializedHeader::PciToCardbusBridge(
                PciToCardbusBridgeHeader::with_pci_cfg(&pci_cfg)?,
            )),
            _ => Err(PciInfoError::UnknownPciHeaderType(header_type)),
        }
    }
}
