use crate::PciInfoError;

use super::pci_config_buffer::PciConfigBuffer;

/// The header for PCI to PCI bus bridges.
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
/// |   0x4    |   0x10  |                Base address #0 (BAR0)                        |
/// |   0x5    |   0x14  |                Base address #1 (BAR1)                        |
/// |   0x6    |   0x18  | S.Lat Timer | Subord. Bus  | Secondary Bus  | Primary Bus    |
/// |   0x7    |   0x1C  |     Secondondary Status    |    I/O Limit   |   I/O Base     |
/// |   0x8    |   0x20  |        Memory Limit        |            Memory Base          |
/// |   0x9    |   0x24  | Prefetchable Memory Limit  |    Prefetchable Memory Base     |
/// |   0xA    |   0x28  |            Prefetchable Base Upper 32 Bits                   |
/// |   0xB    |   0x2C  |            Prefetchable Limit Upper 32 Bits                  |
/// |   0xC    |   0x30  |   I/O Limit Upper 16 Bits  |     I/O Base Upper 16 Bits      |
/// |   0xD    |   0x34  |            Reserved        |      Capability Pointer         |
/// |   0xE    |   0x38  |               Expansion ROM base address                     |
/// |   0xF    |   0x3C  |       Bridge Control       | Interrupt PIN  | Interrupt Line |
/// +----------+---------+-------------+--------------+----------------+----------------+
/// ```
#[derive(Clone, Debug)]
pub struct PciToPciBridgeHeader {
    pub base_addr: [u32; 2],
    pub secondary_latency_timer: u8,
    pub subordinate_bus_number: u8,
    pub secondary_bus_number: u8,
    pub primary_bus_number: u8,
    pub secondary_status: u16,
    pub io_limit: u8,
    pub io_base: u8,
    pub memory_limit: u16,
    pub memory_base: u16,
    pub prefetchable_memory_limit: u16,
    pub prefetchable_memory_base: u16,
    pub prefetchable_base_upper_32_bits: u32,
    pub prefetchable_limit_upper_32_bits: u32,
    pub io_limit_upper_16_bits: u16,
    pub io_base_upper_16_bits: u16,
    pub capability_pointer: u16,
    pub expansion_rom_base_addr: u32,
    pub bridge_control: u16,
    pub interrupt_pin: u8,
    pub interrupt_line: u8,
}

impl PciToPciBridgeHeader {
    pub const LENGTH: usize = 64;
    pub const ID: u8 = 1;

    pub(super) fn with_pci_cfg(pci_cfg: &PciConfigBuffer<'_>) -> Result<Self, PciInfoError> {
        pci_cfg.assert_registers_available(0, 3)?;

        Ok(Self {
            base_addr: [pci_cfg.read_u32(0x4), pci_cfg.read_u32(0x5)],
            secondary_latency_timer: pci_cfg.read_u8(0x6, 3),
            subordinate_bus_number: pci_cfg.read_u8(0x6, 2),
            secondary_bus_number: pci_cfg.read_u8(0x6, 1),
            primary_bus_number: pci_cfg.read_u8(0x6, 0),
            secondary_status: pci_cfg.read_u16_hi(0x7),
            io_limit: pci_cfg.read_u8(0x7, 1),
            io_base: pci_cfg.read_u8(0x7, 0),
            memory_limit: pci_cfg.read_u16_hi(0x8),
            memory_base: pci_cfg.read_u16_lo(0x8),
            prefetchable_memory_limit: pci_cfg.read_u16_hi(0x9),
            prefetchable_memory_base: pci_cfg.read_u16_lo(0x9),
            prefetchable_base_upper_32_bits: pci_cfg.read_u32(0xA),
            prefetchable_limit_upper_32_bits: pci_cfg.read_u32(0xB),
            io_limit_upper_16_bits: pci_cfg.read_u16_hi(0xC),
            io_base_upper_16_bits: pci_cfg.read_u16_lo(0xC),
            capability_pointer: pci_cfg.read_u16_lo(0xD),
            expansion_rom_base_addr: pci_cfg.read_u32(0xE),
            bridge_control: pci_cfg.read_u16_hi(0xF),
            interrupt_pin: pci_cfg.read_u8(0xF, 1),
            interrupt_line: pci_cfg.read_u8(0xF, 0),
        })
    }
}
