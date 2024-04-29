#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A bus number in the PCI hierarchy (i.e. a `segment:bus` tuple).
/// The segment (also called domain in some contexts) is usually zero and
/// unsupported by most enumerators.
pub struct PciBusNumber {
    segment: u16,
    bus: u8,
}

impl PciBusNumber {
    /// Creates a new `PciLocation` with an empty segment
    pub fn new(bus: u8) -> Self {
        Self::with_segment(0, bus)
    }

    pub fn with_segment(segment: u16, bus: u8) -> Self {
        Self { segment, bus }
    }

    /// Gets the bus component of the PCI bus
    pub fn bus(&self) -> u8 {
        self.bus
    }

    /// Gets the segment (sometimes called PCI domain) component
    /// of the PCI bus.
    /// This is zero on most systems as most systems have only
    /// one PCI segment/domain. Also most enumerators do not support
    /// correct deserializing of PCI segments, and systems with
    /// multiple PCI segments are rare enough to make testing very
    /// difficult.
    pub fn segment(&self) -> u16 {
        self.segment
    }
}

impl std::fmt::Debug for PciBusNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}:{:02X}", self.segment, self.bus,)
    }
}

impl std::fmt::Display for PciBusNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}:{:02X}", self.segment, self.bus,)
    }
}
