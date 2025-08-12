// NOTE: This example runs only on Windows as it uses a platform
// specific PCI enumerator.

#[cfg(all(target_os = "windows", feature = "enum_win32_wmi"))]
use pci_info::{enumerators, PciInfo, PciInfoError};

#[cfg(all(target_os = "windows", feature = "enum_win32_wmi"))]
pub fn main() -> Result<(), PciInfoError> {
    // Enumerate the devices by accessing the `enumerate_pci_with_enumerator`
    // method of the `PciInfo` type.
    let info = PciInfo::enumerate_pci_with_enumerator(enumerators::WindowsWmiPciEnumerator)?;

    for r in info {
        match r {
            Ok(device) => println!("{device:?}"),
            Err(error) => eprintln!("{error}"),
        }
    }

    Ok(())
}

#[cfg(not(all(target_os = "windows", feature = "enum_win32_wmi")))]
pub fn main() {
    eprintln!("This example needs Windows to run, and the enum_win32_wmi feature.");
}
