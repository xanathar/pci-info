use crate::PciInfoError;

use super::pci_config_buffer::PciConfigBuffer;

/// The header for generic PCI devices.
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
/// |   0x4    |   0x10  |                 Base address #0 (BAR0)                       |
/// |   0x5    |   0x14  |                 Base address #1 (BAR1)                       |
/// |   0x6    |   0x18  |                 Base address #2 (BAR2)                       |
/// |   0x7    |   0x1C  |                 Base address #3 (BAR3)                       |
/// |   0x8    |   0x20  |                 Base address #4 (BAR4)                       |
/// |   0x9    |   0x24  |                 Base address #5 (BAR5)                       |
/// |   0xA    |   0x28  |                 Cardbus CIS Pointer                          |
/// |   0xB    |   0x2C  |    Subsystem ID            |     Subsystem Vendor ID         |
/// |   0xC    |   0x30  |             Expansion ROM base address                       |
/// |   0xD    |   0x34  |                  Reserved  | Capabilities Pointer            |
/// |   0xE    |   0x38  |                         Reserved                             |
/// |   0xF    |   0x3C  | Max latency |   Min Grant  | Interrupt PIN  | Interrupt Line |
/// +----------+---------+-------------+--------------+----------------+----------------+
/// ```
#[derive(Clone, Debug)]
pub struct PciGenericDeviceHeader {
    pub base_addr: [u32; 6],
    pub cardbus_cis_ptr: u32,
    pub subsystem_device_id: u16,
    pub subsystem_vendor_id: u16,
    pub expansion_rom_base_addr: u32,
    pub capabilities_ptr: u16,
    pub max_latency: u8,
    pub min_grant: u8,
    pub interrupt_pin: u8,
    pub interrupt_line: u8,
}

impl PciGenericDeviceHeader {
    pub const LENGTH: usize = 64;
    pub const ID: u8 = 0;

    pub(super) fn with_pci_cfg(pci_cfg: &PciConfigBuffer<'_>) -> Result<Self, PciInfoError> {
        pci_cfg.assert_registers_available(4, 0xF)?;

        Ok(Self {
            base_addr: [
                pci_cfg.read_u32(0x4),
                pci_cfg.read_u32(0x5),
                pci_cfg.read_u32(0x6),
                pci_cfg.read_u32(0x7),
                pci_cfg.read_u32(0x8),
                pci_cfg.read_u32(0x9),
            ],
            cardbus_cis_ptr: pci_cfg.read_u32(0xA),
            subsystem_device_id: pci_cfg.read_u16_hi(0xB),
            subsystem_vendor_id: pci_cfg.read_u16_lo(0xB),
            expansion_rom_base_addr: pci_cfg.read_u32(0xC),
            capabilities_ptr: pci_cfg.read_u16_hi(0xD),
            max_latency: pci_cfg.read_u8(0xF, 3),
            min_grant: pci_cfg.read_u8(0xF, 2),
            interrupt_pin: pci_cfg.read_u8(0xF, 1),
            interrupt_line: pci_cfg.read_u8(0xF, 0),
        })
    }
}
