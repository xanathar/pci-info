use pci_info::PciInfo;

pub fn main() {
    // Enumerate the devices on the PCI bus using the default
    // enumerator for the current platform. The `unwrap()` panics if
    // the enumeration fatally fails.
    let info = PciInfo::enumerate_pci().unwrap();

    // Print out some properties of the enumerated devices.
    // Note that the collection contains both devices and errors
    // as the enumeration of PCI devices can fail entirely (in which
    // case `PciInfo::enumerate_pci()` would return error) or
    // partially (in which case an error would be inserted in the
    // result).
    for r in info {
        match r {
            Ok(device) => println!("{device:?}"),
            Err(error) => eprintln!("{error}"),
        }
    }
}
