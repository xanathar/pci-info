# pci-info

![CI](https://github.com/xanathar/pci-info/actions/workflows/CI-Rust-Stable.yml/badge.svg) ![Crates.io](https://img.shields.io/crates/v/pci-info) ![docs.rs](https://img.shields.io/docsrs/pci-info) ![Crates.io](https://img.shields.io/crates/d/pci-info) ![Crates.io](https://img.shields.io/crates/l/pci-info)

The `pci-info` crate provides a simple API to enumerate PCI devices across "desktop" operating systems (Linux, Windows, MacOS, with more to be added), or to parse PCI headers from files or memory buffers.

It supports parsing of PCI metadata and availability of the various fields (including possibly the entire standard PCI configuration space of a device); the level of support is subject to the capabilities of the enumerator in use.

It uses user-mode APIs only, accessible from normal users (i.e. no root/Administrator needed). All code has been implemented from scratch through openly available documentation. As such it usually provides less data than some alternative solutions (e.g. libpci) but with less strict requirements (both in terms of licensing and available properties on some platforms).

PCI device classes, subclassses and interface functions are optionally exposed as rusty enums for quick matching, and have been implemented from publicly available documentation.

PCI vendor and device ids are kept as `u16` and have to be manually interpreted or transformed into strings using other crates; the publicly available list of PCI vendors and devices is intentionally not included to contain the crate size (the complete list is large) and to avoid licensing issues (this crate being MIT+Apache dual licensed, the list being GPL+BSD dual licensed).

## Summary of provided features
- Enumeration of devices using OS usermode APIs on Windows, Linux and macOS, with more platforms to be added. See the `PciInfo` type.
- Parsing of PCI headers starting from byte arrays of the PCI configuration space. See the `pci_headers` module.
- Parsing of PCI device classes, subclasses and interface-functions from their codes. See the `pci_enums` module.


# Examples

Using the library with a default enumerator is straightforward:

```rust
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
```

Running this on my system prints:

```
[ 0000:00:00.0 vendor: 1022 device: 1630 revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: None os_driver: 'None']
[ 0000:00:00.2 vendor: 1022 device: 1631 revision: 00 class: 08 sub-class: 06 iface-func: 00 (BaseSystemPeripheral_IOMMU_Default) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(255) os_driver: 'None']
[ 0000:00:01.0 vendor: 1022 device: 1632 revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:01.1 vendor: 1022 device: 1633 revision: 00 class: 06 sub-class: 04 iface-func: 00 (Bridge_PciToPciBridge_PciToPci) subsys-vendor: none subsystem-device: none os_irq: Some(30) os_driver: 'Some("pcieport")']
[ 0000:00:01.2 vendor: 1022 device: 1634 revision: 00 class: 06 sub-class: 04 iface-func: 00 (Bridge_PciToPciBridge_PciToPci) subsys-vendor: none subsystem-device: none os_irq: Some(31) os_driver: 'Some("pcieport")']
[ 0000:00:02.0 vendor: 1022 device: 1632 revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:02.1 vendor: 1022 device: 1634 revision: 00 class: 06 sub-class: 04 iface-func: 00 (Bridge_PciToPciBridge_PciToPci) subsys-vendor: none subsystem-device: none os_irq: Some(32) os_driver: 'Some("pcieport")']
[ 0000:00:02.2 vendor: 1022 device: 1634 revision: 00 class: 06 sub-class: 04 iface-func: 00 (Bridge_PciToPciBridge_PciToPci) subsys-vendor: none subsystem-device: none os_irq: Some(33) os_driver: 'Some("pcieport")']
[ 0000:00:02.4 vendor: 1022 device: 1634 revision: 00 class: 06 sub-class: 04 iface-func: 00 (Bridge_PciToPciBridge_PciToPci) subsys-vendor: none subsystem-device: none os_irq: Some(34) os_driver: 'Some("pcieport")']
[ 0000:00:08.0 vendor: 1022 device: 1632 revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:08.1 vendor: 1022 device: 1635 revision: 00 class: 06 sub-class: 04 iface-func: 00 (Bridge_PciToPciBridge_PciToPci) subsys-vendor: none subsystem-device: none os_irq: Some(35) os_driver: 'Some("pcieport")']
[ 0000:00:14.0 vendor: 1022 device: 790B revision: 51 class: 0C sub-class: 05 iface-func: 00 (SerialBusController_SystemManagementBus_Default) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: None os_driver: 'Some("piix4_smbus")']
[ 0000:00:14.3 vendor: 1022 device: 790E revision: 51 class: 06 sub-class: 01 iface-func: 00 (Bridge_IsaBridge_Default) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: None os_driver: 'None']
[ 0000:00:18.0 vendor: 1022 device: 166A revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:18.1 vendor: 1022 device: 166B revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:18.2 vendor: 1022 device: 166C revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:18.3 vendor: 1022 device: 166D revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'Some("k10temp")']
[ 0000:00:18.4 vendor: 1022 device: 166E revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:18.5 vendor: 1022 device: 166F revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:18.6 vendor: 1022 device: 1670 revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:00:18.7 vendor: 1022 device: 1671 revision: 00 class: 06 sub-class: 00 iface-func: 00 (Bridge_HostBridge_Default) subsys-vendor: 0000 subsystem-device: 0000 os_irq: None os_driver: 'None']
[ 0000:01:00.0 vendor: 10DE device: 249D revision: A1 class: 03 sub-class: 00 iface-func: 00 (DisplayController_VgaCompatible_Vga) subsys-vendor: 1D05 subsystem-device: 1113 os_irq: Some(100) os_driver: 'Some("nvidia")']
[ 0000:01:00.1 vendor: 10DE device: 228B revision: A1 class: 04 sub-class: 03 iface-func: 00 (MultimediaDevice_HdaCompatible_HighDefinitionAudio) subsys-vendor: 1D05 subsystem-device: 1113 os_irq: Some(97) os_driver: 'Some("snd_hda_intel")']
[ 0000:02:00.0 vendor: 10EC device: 8125 revision: 05 class: 02 sub-class: 00 iface-func: 00 (NetworkController_Ethernet_Default) subsys-vendor: 1D05 subsystem-device: 1113 os_irq: Some(61) os_driver: 'Some("r8169")']
[ 0000:03:00.0 vendor: 1987 device: 5012 revision: 01 class: 01 sub-class: 08 iface-func: 02 (MassStorageController_NonVolatileMemory_NVMeIoController) subsys-vendor: 1987 subsystem-device: 5012 os_irq: Some(55) os_driver: 'Some("nvme")']
[ 0000:04:00.0 vendor: 8086 device: 2723 revision: 1A class: 02 sub-class: 80 iface-func: 00 (NetworkController_Other_VendorSpecific) subsys-vendor: 8086 subsystem-device: 0084 os_irq: Some(80) os_driver: 'Some("iwlwifi")']
[ 0000:05:00.0 vendor: 1987 device: 5012 revision: 01 class: 01 sub-class: 08 iface-func: 02 (MassStorageController_NonVolatileMemory_NVMeIoController) subsys-vendor: 1987 subsystem-device: 5012 os_irq: Some(58) os_driver: 'Some("nvme")']
[ 0000:06:00.0 vendor: 1002 device: 1638 revision: C4 class: 03 sub-class: 00 iface-func: 00 (DisplayController_VgaCompatible_Vga) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(45) os_driver: 'Some("amdgpu")']
[ 0000:06:00.2 vendor: 1022 device: 15DF revision: 00 class: 10 sub-class: 80 iface-func: 00 (EncryptionController_Other_VendorSpecific) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(55) os_driver: 'Some("ccp")']
[ 0000:06:00.3 vendor: 1022 device: 1639 revision: 00 class: 0C sub-class: 03 iface-func: 30 (SerialBusController_USB_Usb3Xhci) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(36) os_driver: 'Some("xhci_hcd")']
[ 0000:06:00.4 vendor: 1022 device: 1639 revision: 00 class: 0C sub-class: 03 iface-func: 30 (SerialBusController_USB_Usb3Xhci) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(45) os_driver: 'Some("xhci_hcd")']
[ 0000:06:00.5 vendor: 1022 device: 15E2 revision: 01 class: 04 sub-class: 80 iface-func: 00 (MultimediaDevice_Other_VendorSpecific) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(80) os_driver: 'None']
[ 0000:06:00.6 vendor: 1022 device: 15E3 revision: 00 class: 04 sub-class: 03 iface-func: 00 (MultimediaDevice_HdaCompatible_HighDefinitionAudio) subsys-vendor: 1D05 subsystem-device: 1111 os_irq: Some(98) os_driver: 'Some("snd_hda_intel")']
```

## Using a custom enumerator

If so desired, a custom enumerator can be used, using the
`enumerate_pci_with_enumerator` method of the `PciInfo` type.

For example:

```rust
// NOTE: This example runs only on Windows as it uses a platform
// specific PCI enumerator.

use pci_info::PciInfo;

#[cfg(target_os = "windows")]
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
}
```

Alternatively, you can directly call the `enumerate_pci` of the
`PciEnumerator` trait. The recommended syntax is through the
`PciInfo::enumerate_pci_with_enumerator` method, though.


```rust
// NOTE: This example runs only on Linux as it uses a platform
// specific PCI enumerator.

use pci_info::PciInfo;

#[cfg(target_os = "linux")]
pub fn main() -> Result<(), PciInfoError> {
    // Create a Linux-specific custom enumerator.
    let enumerator = LinuxProcFsPciEnumerator::Fastest;
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
}
```

# Enumerators

Properties provided for PCI devices varies among enumerators.

Enumerator                                     | Platforms | PCI id | PCI location | Revision | Device class | PCI subsystem | Assigned IRQ | OS driver
---------------------------------------------- | --------- | ------ | ------------ | -------- | ------------ | ----------------- | ------------ | ----------
LinuxProcFsPciEnumerator(Fastest)              | Linux   | ✅ | ✅<sup>2</sup> | ❌ | ❌ | ❌ | ✅ | ✅
LinuxProcFsPciEnumerator(HeadersOnly)          | Linux   | ✅ | ✅<sup>2</sup> | ✅ | ✅ | ✅ | ❌ | ❌
LinuxProcFsPciEnumerator(SkipNoncommonHeaders) | Linux   | ✅ | ✅<sup>2</sup> | ✅ | ✅ | ❌ | ✅ | ✅
LinuxProcFsPciEnumerator(Exhaustive)           | Linux   | ✅ | ✅<sup>2</sup> | ✅ | ✅ | ✅ | ✅ | ✅
MacOsIoKitPciEnumerator<sup>3</sup>            | macOS   | ✅ | ⚠️<sup>1, 2</sup> | ✅ | ✅ | ✅ | ❌ | ❌
WindowsSetupApiPciEnumerator                   | Windows | ✅ | ⚠️<sup>1, 2</sup> | ✅ | ✅ | ✅ | ❌ | ❌
WindowsWmiPciEnumerator                        | Windows | ✅ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌

Notes:
- (1) = The PCI location on this enumerator is parsed from human readable strings; that parsing might fail or the information might be incorrect.
- (2) = The PCI location on this enumerator might not support multiple PCI segments/domains correctly.
- (3) = Apparently most of the devices in Apple silicon Macs are not PCI/PCIe. As such PCI enumeration on Apple silicon computers return quite a short list.

# Features

The crate is configurable with the following features:

Crate feature | Default | Description
------------- | ------ | ------------
`pci_class_debug_strings` | YES | Includes human readable debug strings for variants of `pci_enums::PciDeviceClass`. Disable to reduce the binary size.
`pci_subclass_debug_strings` | YES | Includes human readable debug strings for variants of `pci_enums::PciDeviceSubclass`. Disable to reduce the binary size.
`pci_interface_func_debug_string` | YES | Includes human readable debug strings for variants of `pci_enums::PciDeviceInterfaceFunc`. Disable to reduce the binary size.

# Change log

### 0.1.0
First version published with basic enumerators for Linux, Windows and MacOS.

