use std::path::PathBuf;

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
        let (read_headers, read_extended_headers, read_device_file) = self.into_arguments();
        let path = PathBuf::from("/proc/bus");

        #[cfg(target_os = "linux")]
        proc_fs::enumerate_pci(path, read_headers, read_extended_headers, read_device_file)
    }
}

impl LinuxProcFsPciEnumerator {
    /// Creates an enumerator that enumerates PCI devices reading from a copy of the
    /// `/proc/bus/pci` subdirectory. `path` should point to a directory that contains
    /// a `pci/` subdirectory in it.
    pub fn with_custom_path<P>(self, path: P) -> CustomPathLinuxProcFsPciEnumerator
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref().to_owned();
        let (read_headers, read_extended_headers, read_device_file) = self.into_arguments();

        CustomPathLinuxProcFsPciEnumerator {
            path,
            read_device_file,
            read_extended_headers,
            read_headers,
        }
    }

    fn into_arguments(self) -> (bool, bool, bool) {
        match self {
            Self::Fastest => (false, false, true),
            Self::HeadersOnly => (true, true, false),
            Self::SkipNoncommonHeaders => (true, false, true),
            Self::Exhaustive => (true, true, true),
        }
    }
}

/// An enumerator that enumerates PCI devices reading from a copy of
/// the /proc/bus/pci directory. See `LinuxProcFsPciEnumerator::with_custom_path`
/// to build an enumerator of this type.
pub struct CustomPathLinuxProcFsPciEnumerator {
    path: std::path::PathBuf,
    read_headers: bool,
    read_extended_headers: bool,
    read_device_file: bool,
}

impl crate::PciEnumerator for CustomPathLinuxProcFsPciEnumerator {
    fn enumerate_pci(self) -> Result<PciInfo, PciInfoError> {
        #[cfg(target_os = "linux")]
        proc_fs::enumerate_pci(
            self.path,
            self.read_headers,
            self.read_extended_headers,
            self.read_device_file,
        )
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
test_enumerator!(
    LinuxProcFsPciEnumeratorExhaustiveAmd64,
    LinuxProcFsPciEnumerator::Exhaustive.with_custom_path("test-data/linux/amd64")
);
test_enumerator!(
    LinuxProcFsPciEnumeratorExhaustiveAarch64,
    LinuxProcFsPciEnumerator::Exhaustive.with_custom_path("test-data/linux/aarch64")
);
