use crate::{PciInfo, PciInfoError};

#[cfg(target_os = "linux")]
mod proc_fs;

/// A PCI Enumerator for Linux that uses in the
/// virtual `/proc` file system to extract PCI data
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LinuxProcFsPciEnumerator {
    /// Enumerates the PCI devices in the fastest way possible,
    /// returning only PCI locations, vendor and device ids and
    /// some OS related data. Data is read from the `/proc/bus/pci/devices`
    /// file.
    Fastest,
    /// Enumerates the PCI devices using only the PCI configuration
    /// contained in `/proc/bus/pci` subdirectories. Does not parse
    /// the `/proc/bus/pci/devices` file.
    HeadersOnly,
    /// Enumerates the PCI devices using the PCI configuration
    /// contained in `/proc/bus/pci` subdirectories and the
    /// `/proc/bus/pci/devices` file. Data from specific PCI headers
    /// is not parsed and some fields might be missing (most notably,
    /// the subsystem ids).
    SkipNoncommonHeaders,
    /// Enumerates the PCI devices using the PCI configuration
    /// contained in `/proc/bus/pci` subdirectories and the
    /// `/proc/bus/pci/devices` file. This option provides the most
    /// extensive data about PCI devices.
    Exhaustive,
}

impl crate::PciEnumerator for LinuxProcFsPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        let (read_headers, read_extended_headers, read_device_file) = match self {
            Self::Fastest => (false, false, true),
            Self::HeadersOnly => (true, true, false),
            Self::SkipNoncommonHeaders => (true, false, true),
            Self::Exhaustive => (true, true, true),
        };

        #[cfg(target_os = "linux")]
        proc_fs::enumerate_pci(read_headers, read_extended_headers, read_device_file)
    }
}

test_enumerator!(
    LinuxProcFsPciEnumeratorFastest,
    LinuxProcFsPciEnumerator::Fastest
);
test_enumerator!(
    LinuxProcFsPciEnumeratorExhaustive,
    LinuxProcFsPciEnumerator::Exhaustive
);
test_enumerator!(
    LinuxProcFsPciEnumeratorHeadersOnly,
    LinuxProcFsPciEnumerator::HeadersOnly
);
test_enumerator!(
    LinuxProcFsPciEnumeratorSkipNoncommonHeaders,
    LinuxProcFsPciEnumerator::SkipNoncommonHeaders
);
