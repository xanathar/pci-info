use crate::{
    pci_device::PciDeviceProperties, pci_info::PciInfo, pci_property_result::PropertyResult,
    PciDevice, PciDeviceEnumerationError, PciDeviceEnumerationErrorImpact, PciInfoError,
    PciLocation,
};
use core_foundation::{
    base::kCFAllocatorDefault, base::CFAllocatorRef, base::CFRange, base::CFRelease, base::TCFType,
    data::CFDataGetBytes, data::CFDataRef, dictionary::CFDictionaryGetValue,
    dictionary::CFDictionaryRef, dictionary::CFMutableDictionaryRef, string::CFString,
    string::CFStringRef,
};
use std::ffi::{c_char, c_int};

#[link(name = "IoKit", kind = "framework")]
extern "C" {
    fn IOObjectRelease(object: IORegEntry) -> KernReturn;
    fn IORegistryEntryCreateCFProperties(
        entry: IORegEntry,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> KernReturn;
    fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    fn IOServiceGetMatchingServices(
        port: MachPort,
        matching: CFDictionaryRef,
        existing: *mut IoIterator,
    ) -> KernReturn;
    fn IOIteratorNext(iterator: IoIterator) -> IOObject;
}

type KernReturn = c_int;
type MachPort = u32;
type IoIterator = u32;
type IOObject = u32;
type IORegEntry = IOObject;
type IOOptionBits = u32;
const IOREG_IOMASTERPORTDEFAULT: MachPort = 0;
const KERN_SUCCESS: KernReturn = 0;
const IO_OBJECT_NULL: IOObject = 0;
#[allow(non_upper_case_globals)]
const kNilOptions: IOOptionBits = 0;

unsafe fn get_property<T: Copy + Default>(
    properties: CFMutableDictionaryRef,
    key: &'static str,
) -> Result<T, PciInfoError> {
    let cfkey = CFString::new(key);
    let property_data: CFDataRef =
        CFDictionaryGetValue(properties, cfkey.as_concrete_TypeRef() as *const _) as _;

    if !property_data.is_null() {
        let mut value: T = T::default();
        let size_of_t = std::mem::size_of::<T>() as isize;

        // We test for coherence in IOKit property sizes only in test to
        // avoid runtime panics. These are tested though.
        #[cfg(test)]
        {
            let property_len = core_foundation::data::CFDataGetLength(property_data);
            assert_eq!(
                property_len, size_of_t,
                "{key} size mismatch: property = {property_len}, type = {size_of_t}"
            );
        }

        CFDataGetBytes(
            property_data,
            CFRange::init(0, size_of_t),
            &mut value as *mut T as *mut _,
        );
        Ok(value)
    } else {
        Err(PciInfoError::ValueNotFound(Some(key.to_owned().into())))
    }
}

unsafe fn get_str_property(
    properties: CFMutableDictionaryRef,
    key: &'static str,
) -> Result<String, PciInfoError> {
    let cfkey = CFString::new(key);
    let property_str: CFStringRef =
        CFDictionaryGetValue(properties, cfkey.as_concrete_TypeRef() as *const _) as _;

    if !property_str.is_null() {
        let property_string = CFString::wrap_under_get_rule(property_str);
        Ok(property_string.to_string())
    } else {
        Err(PciInfoError::ValueNotFound(Some(key.to_owned().into())))
    }
}

unsafe fn enrich_and_append_device(
    pi: &mut PciInfo,
    mut dev: PciDevice,
    properties: *mut core_foundation::dictionary::__CFDictionary,
) {
    // subsystem ids are known to fail, we consider them valid
    dev.properties.subsystem_vendor_id.set_val(
        get_property::<u32>(properties, "subsystem-vendor-id")
            .ok()
            .and_then(|v| v.try_into().ok()),
    );
    dev.properties.subsystem_device_id.set_val(
        get_property::<u32>(properties, "subsystem-id")
            .ok()
            .and_then(|v| v.try_into().ok()),
    );

    dev.properties
        .revision
        .set_res_cast(get_property::<u32>(properties, "revision-id"));

    match get_property::<u32>(properties, "class-code") {
        Ok(class_triplet) => {
            let class_triplet = decompose_class_code(class_triplet);
            dev.properties.device_class.set_val(class_triplet.0);
            dev.properties.device_subclass.set_val(class_triplet.1);
            dev.properties.device_iface.set_val(class_triplet.2);
        }
        Err(e) => {
            dev.properties.device_class.set_err(e.clone());
            dev.properties.device_subclass.set_err(e.clone());
            dev.properties.device_iface.set_err(e);
        }
    }

    pi.push_device(dev);
}

unsafe fn create_device(properties: CFMutableDictionaryRef) -> Result<PciDevice, PciInfoError> {
    let vendor_id = get_property::<u32>(properties, "vendor-id")? as u16;
    let device_id = get_property::<u32>(properties, "device-id")? as u16;

    Ok(PciDevice::new(
        vendor_id,
        device_id,
        PciDeviceProperties {
            location: PropertyResult::with_res(try_parse_location_from_pcidebug(properties)),
            ..Default::default()
        },
    ))
}

unsafe fn try_parse_location_from_pcidebug(
    properties: CFMutableDictionaryRef,
) -> Result<PciLocation, PciInfoError> {
    let pcidebug = get_str_property(properties, "pcidebug")?;

    let pcidbg = pcidebug.split_once('(').map(|s| s.0).unwrap_or(&pcidebug);
    let parts = pcidbg.split(':').collect::<Vec<_>>();

    if parts.len() == 3 {
        let Ok(bus) = parts[0].parse::<u8>() else {
            return Err(PciInfoError::ParseError(
                format!("Failed to parse '{pcidebug}' as a PCI location").into(),
            ));
        };
        let Ok(dev) = parts[1].parse::<u8>() else {
            return Err(PciInfoError::ParseError(
                format!("Failed to parse '{pcidebug}' as a PCI location").into(),
            ));
        };
        let Ok(fun) = parts[2].parse::<u8>() else {
            return Err(PciInfoError::ParseError(
                format!("Failed to parse '{pcidebug}' as a PCI location").into(),
            ));
        };

        PciLocation::with_bdf(bus, dev, fun)
    } else {
        Err(PciInfoError::ParseError(
            format!("Failed to parse '{pcidebug}' as a PCI location").into(),
        ))
    }
}

fn decompose_class_code(class_code: u32) -> (u8, u8, u8) {
    (
        ((class_code & 0xFF0000) >> 16) as u8,
        ((class_code & 0xFF00) >> 8) as u8,
        (class_code & 0xFF) as u8,
    )
}

pub(super) fn enumerate_pci() -> Result<PciInfo, PciInfoError> {
    let mut pi = PciInfo::empty();

    unsafe {
        let mut iterator: IoIterator = 0;
        let matching = IOServiceMatching(b"IOPCIDevice\0" as *const u8 as *const _);

        let kr = IOServiceGetMatchingServices(IOREG_IOMASTERPORTDEFAULT, matching, &mut iterator);

        if kr != KERN_SUCCESS {
            CFRelease(matching as *const _);
            return Err(PciInfoError::IoKitError(kr));
        }

        loop {
            let registry_entry = IOIteratorNext(iterator);

            if registry_entry == IO_OBJECT_NULL {
                break;
            }

            let mut properties: CFMutableDictionaryRef = std::ptr::null_mut();

            let kr = IORegistryEntryCreateCFProperties(
                registry_entry,
                &mut properties,
                kCFAllocatorDefault,
                kNilOptions,
            );

            if kr == KERN_SUCCESS && !properties.is_null() {
                match create_device(properties) {
                    Ok(dev) => enrich_and_append_device(&mut pi, dev, properties),
                    Err(e) => pi.push_error(PciDeviceEnumerationError::new(
                        PciDeviceEnumerationErrorImpact::Device,
                        e,
                    )),
                };

                CFRelease(properties as *const _);
            } else {
                pi.push_error(PciDeviceEnumerationError::new(
                    PciDeviceEnumerationErrorImpact::Device,
                    PciInfoError::IoKitError(kr),
                ));
                continue;
            }

            IOObjectRelease(registry_entry);
        }
    }

    Ok(pi)
}
