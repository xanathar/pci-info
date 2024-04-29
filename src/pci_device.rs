// Dead code is allowed in this module as it serves multiple platforms
#![allow(dead_code)]

use crate::pci_property_result::PropertyResult;
use crate::{
    pci_enums::{PciDeviceClass, PciDeviceInterfaceFunc, PciDeviceSubclass},
    pci_headers::{PciCommonHeader, PciSpecializedHeader},
    PciInfoError, PciInfoPropertyError, PciLocation,
};
use std::fmt;

/// A struct representing the data of a PCI device as discovered through
/// the OS APIs by the enumerator in use. An enumerator is not required
/// to fill all the fields of a `PciDevice` object. As such most members
/// are optional and only the `vendor_id()`  and `device_id()` are
/// required to be valid. Check the documentation of the enumerator in
/// use to see what values are expected to be filled.
pub struct PciDevice {
    vendor_id: u16,
    device_id: u16,
    pub(crate) properties: PciDeviceProperties,
}

#[derive(Default)]
pub(crate) struct PciDeviceProperties {
    pub(crate) location: PropertyResult<PciLocation>,
    pub(crate) subsystem_vendor_id: PropertyResult<Option<u16>>,
    pub(crate) subsystem_device_id: PropertyResult<Option<u16>>,
    pub(crate) revision: PropertyResult<u8>,
    pub(crate) device_class: PropertyResult<u8>,
    pub(crate) device_subclass: PropertyResult<u8>,
    pub(crate) device_iface: PropertyResult<u8>,
    pub(crate) os_irq: PropertyResult<Option<u8>>,
    pub(crate) os_driver: PropertyResult<Option<String>>,
    pub(crate) pci_common_header: PropertyResult<PciCommonHeader>,
    pub(crate) pci_specialized_header: PropertyResult<PciSpecializedHeader>,
}

impl PciDevice {
    pub(crate) fn new(vendor_id: u16, device_id: u16, properties: PciDeviceProperties) -> Self {
        Self {
            vendor_id,
            device_id,
            properties,
        }
    }

    pub fn from_pci_header_set(
        header: PciCommonHeader,
        specialized: Option<PciSpecializedHeader>,
    ) -> Self {
        let (subsystem_device_id, subsystem_vendor_id, pci_specialized_header) = match specialized {
            Some(PciSpecializedHeader::GenericDevice(h)) => (
                PropertyResult::with_val(Some(h.subsystem_device_id)),
                PropertyResult::with_val(Some(h.subsystem_vendor_id)),
                PropertyResult::with_val(PciSpecializedHeader::GenericDevice(h)),
            ),
            Some(PciSpecializedHeader::PciToCardbusBridge(h)) => (
                PropertyResult::with_val(Some(h.subsystem_device_id)),
                PropertyResult::with_val(Some(h.subsystem_vendor_id)),
                PropertyResult::with_val(PciSpecializedHeader::PciToCardbusBridge(h)),
            ),
            Some(PciSpecializedHeader::PciToPciBridge(h)) => (
                PropertyResult::with_val(None),
                PropertyResult::with_val(None),
                PropertyResult::with_val(PciSpecializedHeader::PciToPciBridge(h)),
            ),
            None => (
                PropertyResult::default(),
                PropertyResult::default(),
                PropertyResult::default(),
            ),
        };

        Self {
            vendor_id: header.vendor_id,
            device_id: header.device_id,
            properties: PciDeviceProperties {
                revision: PropertyResult::with_val(header.revision_id),
                device_class: PropertyResult::with_val(header.class_code),
                device_subclass: PropertyResult::with_val(header.subclass_code),
                device_iface: PropertyResult::with_val(header.prog_iface_code),
                subsystem_device_id,
                subsystem_vendor_id,
                pci_common_header: PropertyResult::with_val(header),
                pci_specialized_header,
                ..Default::default()
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_pci_header_result(
        header: PciCommonHeader,
        specialized: Result<PciSpecializedHeader, PciInfoError>,
    ) -> Self {
        let (subsystem_device_id, subsystem_vendor_id, pci_specialized_header) = match specialized {
            Ok(PciSpecializedHeader::GenericDevice(h)) => (
                PropertyResult::with_val(Some(h.subsystem_device_id)),
                PropertyResult::with_val(Some(h.subsystem_vendor_id)),
                PropertyResult::with_val(PciSpecializedHeader::GenericDevice(h)),
            ),
            Ok(PciSpecializedHeader::PciToCardbusBridge(h)) => (
                PropertyResult::with_val(Some(h.subsystem_device_id)),
                PropertyResult::with_val(Some(h.subsystem_vendor_id)),
                PropertyResult::with_val(PciSpecializedHeader::PciToCardbusBridge(h)),
            ),
            Ok(PciSpecializedHeader::PciToPciBridge(h)) => (
                PropertyResult::with_val(None),
                PropertyResult::with_val(None),
                PropertyResult::with_val(PciSpecializedHeader::PciToPciBridge(h)),
            ),
            Err(e) => (
                PropertyResult::with_err(e.clone()),
                PropertyResult::with_err(e.clone()),
                PropertyResult::with_err(e.clone()),
            ),
        };

        Self {
            vendor_id: header.vendor_id,
            device_id: header.device_id,
            properties: PciDeviceProperties {
                revision: PropertyResult::with_val(header.revision_id),
                device_class: PropertyResult::with_val(header.class_code),
                device_subclass: PropertyResult::with_val(header.subclass_code),
                device_iface: PropertyResult::with_val(header.prog_iface_code),
                subsystem_device_id,
                subsystem_vendor_id,
                pci_common_header: PropertyResult::with_val(header),
                pci_specialized_header,
                ..Default::default()
            },
        }
    }

    /// Returns the id of the vendor of this device. The vendor is usually
    /// the provider of the chipset or technology upon which the device is
    /// based.
    ///
    /// For example, the 3D Blaster Banshee is a graphic card from Diamond
    /// Multimedia based on 3dfx Banshee chipset.
    /// The `subsystem_vendor_id` for that card is the one for Diamond Multimedia,
    /// while `vendor_id` is the one for 3dfx, the provider of the graphic chipset.
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// Returns the device id of the device.
    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    /// Returs an optional `PciLocation` object that contains the location of
    /// the device on the PCI bus (in terms of bus, device and function)
    pub fn location(&self) -> Result<PciLocation, &PciInfoPropertyError> {
        self.properties.location.as_result()
    }

    /// Returns the vendor id of the manufacturer of this device. The vendor is usually
    /// the provider of the actual card/device.
    ///
    /// For example, the 3D Blaster Banshee is a graphic card from Diamond
    /// Multimedia based on 3dfx Banshee chipset.
    /// The `subsystem_vendor_id` for that card is the one for Diamond Multimedia,
    /// while `vendor_id` is the one for 3dfx, the provider of the graphic chipset.
    pub fn subsystem_vendor_id(&self) -> Result<Option<u16>, &PciInfoPropertyError> {
        self.properties.subsystem_vendor_id.as_result()
    }

    /// Returns the `subsystem_device_id`. See the documentation for
    /// `subsystem_vendor_id` for the difference between normal ids and
    /// subsystem ids.
    pub fn subsystem_device_id(&self) -> Result<Option<u16>, &PciInfoPropertyError> {
        self.properties.subsystem_device_id.as_result()
    }

    /// Returns the revision of the device.
    pub fn revision(&self) -> Result<u8, &PciInfoPropertyError> {
        self.properties.revision.as_result()
    }

    /// Returns the code of the PCI device class.
    /// Use `device_class` if an intelligible enumeration
    /// is preferred.
    pub fn device_class_code(&self) -> Result<u8, &PciInfoPropertyError> {
        self.properties.device_class.as_result()
    }

    /// Returns the code of the PCI device subclass.
    /// Use `device_subclass` if an intelligible enumeration
    /// is preferred.
    pub fn device_subclass_code(&self) -> Result<u8, &PciInfoPropertyError> {
        self.properties.device_subclass.as_result()
    }

    /// Returns the code of the PCI interface function.
    /// Use `device_iface` if an intelligible enumeration
    /// is preferred.
    pub fn device_iface_code(&self) -> Result<u8, &PciInfoPropertyError> {
        self.properties.device_iface.as_result()
    }

    /// Returns the PCI device class of this device.
    pub fn device_class(&self) -> Result<PciDeviceClass, &PciInfoPropertyError> {
        Ok(PciDeviceClass::from_code(self.device_class_code()?))
    }

    /// Returns the PCI device subclass of this device.
    pub fn device_subclass(&self) -> Result<PciDeviceSubclass, &PciInfoPropertyError> {
        Ok(PciDeviceSubclass::from_codes(
            self.device_class_code()?,
            self.device_subclass_code()?,
        ))
    }

    /// Returns the PCI interface function of this device.
    pub fn device_iface(&self) -> Result<PciDeviceInterfaceFunc, &PciInfoPropertyError> {
        Ok(PciDeviceInterfaceFunc::from_codes(
            self.device_class_code()?,
            self.device_subclass_code()?,
            self.device_iface_code()?,
        ))
    }

    /// Returns the IRQ that the OS has associated to the device.
    pub fn os_irq(&self) -> Result<Option<u8>, &PciInfoPropertyError> {
        self.properties.os_irq.as_result()
    }

    /// Returns the name of the driver that handles the device in the OS.
    pub fn os_driver(&self) -> Result<&Option<String>, &PciInfoPropertyError> {
        self.properties.os_driver.as_result_ref()
    }

    // Returns the common part of the PCI Configuration space header for this device.
    pub fn pci_common_header(&self) -> Result<&PciCommonHeader, &PciInfoPropertyError> {
        self.properties.pci_common_header.as_result_ref()
    }

    // Returns the specialied part of the PCI Configuration space header for this device.
    pub fn pci_specialized_header(&self) -> Result<&PciSpecializedHeader, &PciInfoPropertyError> {
        self.properties.pci_specialized_header.as_result_ref()
    }
}

impl fmt::Debug for PciDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        write!(f, " {:?}", &self.properties.location)?;
        write!(f, " vendor: {:04X}", self.vendor_id)?;
        write!(f, " device: {:04X}", self.device_id)?;
        write!(f, " revision: {:?}", self.properties.revision)?;
        write!(f, " class: {:?}", self.properties.device_class)?;
        write!(f, " sub-class: {:?}", self.properties.device_subclass)?;
        write!(f, " iface-func: {:?}", self.properties.device_iface)?;

        match (
            self.properties.device_class.as_result(),
            self.properties.device_subclass.as_result(),
            self.properties.device_iface.as_result(),
        ) {
            (Ok(c), Ok(s), Ok(i)) => {
                write!(f, " ({:?})", PciDeviceInterfaceFunc::from_codes(c, s, i))?
            }
            (Ok(c), Ok(s), _) => write!(f, " ({:?})", PciDeviceSubclass::from_codes(c, s))?,
            (Ok(c), _, _) => write!(f, " ({:?})", PciDeviceClass::from_code(c))?,
            _ => (),
        }

        write!(
            f,
            " subsys-vendor: {:?}",
            self.properties.subsystem_vendor_id
        )?;
        write!(
            f,
            " subsystem-device: {:?}",
            self.properties.subsystem_device_id
        )?;

        if let Ok(v) = self.os_irq() {
            write!(f, " os_irq: {:?}", v)?;
        }

        if let Ok(v) = self.os_driver() {
            write!(f, " os_driver: '{:?}'", v)?;
        }

        write!(f, "]")
    }
}
