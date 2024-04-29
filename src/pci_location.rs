use crate::{PciBusNumber, PciInfoError};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A location in the PCI hierarchy (i.e. a `segment:bus:device.function` tuple).
/// The segment (also called domain in some contexts) is usually zero and
/// unsupported by most enumerators.
pub struct PciLocation {
    segment: u16,
    bus: u8,
    device: u8,
    function: u8,
}

impl PciLocation {
    /// Creates a new `PciLocation` with an empty segment
    pub fn with_bdf(bus: u8, device: u8, function: u8) -> Result<Self, PciInfoError> {
        Self::with_segment(0, bus, device, function)
    }

    /// Creates a new `PciLocation` with an empty segment starting from
    /// a PCI location expressed as a 16bit integer
    pub fn with_bdf_u16(bdf: u16) -> Self {
        Self {
            segment: 0,
            bus: (bdf >> 8) as u8,
            device: ((bdf & 0b_1111_1000) >> 3) as u8,
            function: (bdf & 0b_111) as u8,
        }
    }

    /// Creates a new `PciLocation` also specifying the PCI segment
    pub fn with_segment(
        segment: u16,
        bus: u8,
        device: u8,
        function: u8,
    ) -> Result<Self, PciInfoError> {
        if device >= 32 || function >= 8 {
            Err(PciInfoError::BdfLocationOutOfRange(bus, device, function))
        } else {
            Ok(Self {
                segment,
                bus,
                device,
                function,
            })
        }
    }

    /// Gets the bus component of the PCI location
    pub fn bus(&self) -> u8 {
        self.bus
    }

    /// Gets the bus component of the PCI location as a `PciBusNumber`
    pub fn bus_number(&self) -> PciBusNumber {
        PciBusNumber::with_segment(self.segment, self.bus)
    }

    /// Gets the device component of the PCI location
    pub fn device(&self) -> u8 {
        self.device
    }

    /// Gets the function component of the PCI location
    pub fn function(&self) -> u8 {
        self.function
    }

    /// Gets the segment (sometimes called PCI domain) component
    /// of the PCI location.
    /// This is zero on most systems as most systems have only
    /// one PCI segment/domain. Also most enumerators do not support
    /// correct deserializing of PCI segments, and systems with
    /// multiple PCI segments are rare enough to make testing very
    /// difficult.
    pub fn segment(&self) -> u16 {
        self.segment
    }
}

impl std::fmt::Debug for PciLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04X}:{:02X}:{:02X}.{:X}",
            self.segment, self.bus, self.device, self.function
        )
    }
}

impl std::fmt::Display for PciLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04X}:{:02X}:{:02X}.{:X}",
            self.segment, self.bus, self.device, self.function
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bdf_validation() {
        assert!(PciLocation::with_bdf(0, 0, 0).is_ok());
        assert!(PciLocation::with_bdf(0, 31, 7).is_ok());
        assert!(PciLocation::with_bdf(0, 32, 0).is_err());
        assert!(PciLocation::with_bdf(0, 0, 8).is_err());
    }
}
