use crate::PciInfoError;

use super::pci_config_buffer::PciConfigBuffer;

/// The header that is common for all PCI devices.
///
/// All fields are the raw values of every non-reserved register part in
/// the configuration space.
///
/// The format of the header in PCI configuration space is the following.
///
///  ```text
/// +----------+---------+-------------+--------------+----------------+----------------+
/// | Register | Offset  | Bits 31-24  | Bits 23-16   | Bits 15-8      | Bits 7-0       |
/// +----------+---------+-------------+--------------+----------------+----------------+
/// |    0x0   |   0x0   | Device ID                  | Vendor ID                       |
/// |    0x1   |   0x4   | Status                     | Command                         |
/// |    0x2   |   0x8   | Class code  | Subclass     | Prog IF        | Revision ID    |
/// |    0x3   |   0xC   | BIST        | Header type  | Latency Timer  | Cache Line Size|
/// +----------+---------+-------------+--------------+----------------+----------------+
/// ```
#[derive(Clone, Debug)]
pub struct PciCommonHeader {
    pub device_id: u16,
    pub vendor_id: u16,
    pub status: u16,
    pub command: u16,
    pub class_code: u8,
    pub subclass_code: u8,
    pub prog_iface_code: u8,
    pub revision_id: u8,
    pub bist: u8,
    pub header_type: u8,
    pub latency_timer: u8,
    pub cache_line_size: u8,
}

impl PciCommonHeader {
    /// The maximum legth of the data of PCI headers parsed by this
    /// crate.
    pub const MAX_HEADER_LEN: usize = super::PciToCardbusBridgeHeader::LENGTH;
    /// The length of the common header in PCI configuration space.
    pub const COMMON_HEADER_LEN: usize = 16;

    /// Reads the common header from a slice containing the bytes of
    /// the PCI configuration space for this device.
    pub fn with_bytes(bytes: &[u8]) -> Result<Self, PciInfoError> {
        Self::with_pci_cfg(&PciConfigBuffer::new(bytes, 0))
    }

    fn with_pci_cfg(pci_cfg: &PciConfigBuffer<'_>) -> Result<Self, PciInfoError> {
        pci_cfg.assert_registers_available(0, 3)?;

        Ok(Self {
            vendor_id: pci_cfg.read_u16_lo(0),
            device_id: pci_cfg.read_u16_hi(0),
            command: pci_cfg.read_u16_lo(1),
            status: pci_cfg.read_u16_hi(1),
            revision_id: pci_cfg.read_u8(2, 0),
            prog_iface_code: pci_cfg.read_u8(2, 1),
            subclass_code: pci_cfg.read_u8(2, 2),
            class_code: pci_cfg.read_u8(2, 3),
            cache_line_size: pci_cfg.read_u8(3, 0),
            latency_timer: pci_cfg.read_u8(3, 1),
            header_type: pci_cfg.read_u8(3, 2),
            bist: pci_cfg.read_u8(3, 3),
        })
    }
}
