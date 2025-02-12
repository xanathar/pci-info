#![cfg(feature = "enum_win32_wmi")]

use std::collections::HashMap;
use wmi::Variant;
use wmi::*;

use super::common::*;
use crate::{pci_info::PciInfo, PciDeviceEnumerationError, PciInfoError};

pub(super) fn enumerate_pci() -> Result<PciInfo, PciInfoError> {
    let mut pi = PciInfo::empty();

    let wmi_con = WMIConnection::new(COMLibrary::new()?)?;
    let devices: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT * FROM Win32_PnPEntity")?;

    for device in devices {
        let Some(Variant::String(devid)) = device.get("DeviceID") else {
            continue;
        };

        if !devid.starts_with("PCI\\") {
            continue;
        }

        let result = read_device_from_wmi_properties(device);

        match result {
            Ok(d) => pi.results.push(d.try_into_device(false).map_err(|e| {
                PciDeviceEnumerationError::new(crate::PciDeviceEnumerationErrorImpact::Device, e)
            })),
            Err(e) => pi.push_error(PciDeviceEnumerationError::new(
                crate::PciDeviceEnumerationErrorImpact::Device,
                e,
            )),
        };
    }

    Ok(pi)
}

fn read_device_from_wmi_properties(
    device: HashMap<String, Variant>,
) -> Result<DeviceEntry, PciInfoError> {
    let Some(Variant::Array(hwids)) = device.get("HardwareID") else {
        return Err(PciInfoError::ValueNotFound(Some("HardwareID".into())));
    };

    let Some(Variant::Array(compids)) = device.get("CompatibleID") else {
        return Err(PciInfoError::ValueNotFound(Some("CompatibleID".into())));
    };

    let mut de = DeviceEntry::default();

    for hwid in hwids.iter().chain(compids.iter()) {
        let Variant::String(hwid) = hwid else {
            continue;
        };

        if !hwid.starts_with("PCI\\") {
            continue;
        }

        let (_, hwid) = hwid.split_at(4);

        try_parse_windows_hwid(&mut de, hwid);
    }

    Ok(de)
}
