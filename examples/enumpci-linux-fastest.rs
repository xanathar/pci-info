// NOTE: This example runs only on Linux as it uses a platform
// specific PCI enumerator.

#[cfg(target_os = "linux")]
use pci_info::{enumerators, enumerators::PciEnumerator, PciInfoError};

#[cfg(target_os = "linux")]
pub fn main() -> Result<(), PciInfoError> {
    // Create a Linux-specific custom enumerator.
    let enumerator = enumerators::LinuxProcFsPciEnumerator::Fastest;
    // Enumerate the devices by accessing the `enumerate_pci`
    // method of the `PciEnumerator` trait. Works but using
    // `PciInfo::enumerate_pci_with_enumerator` is preferred.
    let info = enumerator.enumerate_pci()?;

    for r in info {
        match r {
            Ok(device) => println!("{device:?}"),
            Err(error) => eprintln!("{error}"),
        }
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn main() {
    eprintln!("This example needs Linux to run.");
}
