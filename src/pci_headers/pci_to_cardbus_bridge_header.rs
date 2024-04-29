use crate::PciInfoError;

use super::pci_config_buffer::PciConfigBuffer;

/// The header for PCI to Cardbus bus bridges.
///
/// All fields are the raw values of every non-reserved register part in
/// the configuration space.
///
/// The format of the header in PCI configuration space is the following.
///
///  ```text
/// +----------+---------+-------------+--------------+----------------+----------------+
/// | Register | Offset  |  Bits 31-24 | Bits 23-16   | Bits 15-8      | Bits 7-0       |
/// +----------+---------+-------------+--------------+----------------+----------------+
/// |    0x4   |   0x10  |              CardBus Socket/ExCa base address                |
/// |    0x5   |   0x14  |     Secondary status       | Reserved       | Offset of caps |
/// |    0x6   |   0x18  | Cbus Lat tim| Subord. bus  | CardBus bus r  | PCI bus number |
/// |    0x7   |   0x1C  |                 Memory Base Address 0                        |
/// |    0x8   |   0x20  |                 Memory Limit 0                               |
/// |    0x9   |   0x24  |                 Memory Base Address 1                        |
/// |    0xA   |   0x28  |                 Memory Limit 1                               |
/// |    0xB   |   0x2C  |                 I/O Base Address 0                           |
/// |    0xC   |   0x30  |                 I/O Limit 0                                  |
/// |    0xD   |   0x34  |                 I/O Base Address 1                           |
/// |    0xE   |   0x38  |                 I/O Limit 1                                  |
/// |    0xF   |   0x3C  |       Bridge Control       | Interrupt PIN  | Interrupt Line |
/// |   0x10   |   0x40  |    Subsystem Vendor ID     |       Subsystem Device ID       |
/// |   0x11   |   0x44  |        16-bit PC Card legacy mode base address               |
/// +----------+---------+-------------+--------------+----------------+----------------+
/// ```
#[derive(Clone, Debug)]
pub struct PciToCardbusBridgeHeader {
    pub cardbus_socket_exca_base_addr: u32,
    pub secondary_status: u16,
    pub offset_of_capabilities_list: u8,
    pub cardbus_latency_timer: u8,
    pub subordinate_bus_number: u8,
    pub cardbus_bus_number: u8,
    pub pci_bus_number: u8,
    pub memory_base_addr_0: u32,
    pub memory_limit_0: u32,
    pub memory_base_addr_1: u32,
    pub memory_limit_1: u32,
    pub io_base_addr_0: u32,
    pub io_limit_0: u32,
    pub io_base_addr_1: u32,
    pub io_limit_1: u32,
    pub bridge_control: u16,
    pub interrupt_pin: u8,
    pub interrupt_line: u8,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub pc_card_16bit_legacy_mode_base_addr: u32,
}

impl PciToCardbusBridgeHeader {
    pub const LENGTH: usize = 72;
    pub const ID: u8 = 2;

    pub(super) fn with_pci_cfg(pci_cfg: &PciConfigBuffer<'_>) -> Result<Self, PciInfoError> {
        pci_cfg.assert_registers_available(0, 3)?;

        Ok(Self {
            cardbus_socket_exca_base_addr: pci_cfg.read_u32(0x4),
            secondary_status: pci_cfg.read_u16_hi(0x5),
            offset_of_capabilities_list: pci_cfg.read_u8(0x5, 0),
            cardbus_latency_timer: pci_cfg.read_u8(0x6, 3),
            subordinate_bus_number: pci_cfg.read_u8(0x6, 2),
            cardbus_bus_number: pci_cfg.read_u8(0x6, 1),
            pci_bus_number: pci_cfg.read_u8(0x6, 0),
            memory_base_addr_0: pci_cfg.read_u32(0x7),
            memory_limit_0: pci_cfg.read_u32(0x8),
            memory_base_addr_1: pci_cfg.read_u32(0x9),
            memory_limit_1: pci_cfg.read_u32(0xA),
            io_base_addr_0: pci_cfg.read_u32(0xB),
            io_limit_0: pci_cfg.read_u32(0xC),
            io_base_addr_1: pci_cfg.read_u32(0xD),
            io_limit_1: pci_cfg.read_u32(0xE),
            bridge_control: pci_cfg.read_u16_hi(0xF),
            interrupt_pin: pci_cfg.read_u8(0xF, 1),
            interrupt_line: pci_cfg.read_u8(0xF, 0),
            subsystem_vendor_id: pci_cfg.read_u16_hi(0x10),
            subsystem_device_id: pci_cfg.read_u16_lo(0x10),
            pc_card_16bit_legacy_mode_base_addr: pci_cfg.read_u32(0x11),
        })
    }
}
