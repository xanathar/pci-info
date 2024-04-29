use std::mem;
use std::ptr;
use windows::core::GUID;
use windows::core::HRESULT;
use windows::core::PCWSTR;
use windows::Win32::Devices::DeviceAndDriverInstallation as setupapi;
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Foundation::ERROR_INVALID_DATA;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Registry::*;

use super::common::*;
use crate::PciDeviceEnumerationError;
use crate::PciLocation;
use crate::{pci_info::PciInfo, PciInfoError};

const MAX_DEVICE_ID_LEN: usize = 16384;

static PCI_PREFIX: &[u16] = &['P' as u16, 'C' as u16, 'I' as u16, '\\' as u16];

static GUID_DEVCLASS_SYSTEM: GUID = GUID {
    data1: 0x4d36e97d,
    data2: 0xe325,
    data3: 0x11ce,
    data4: [0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18],
};

#[derive(Debug)]
#[allow(dead_code)]
enum RegistryProperty {
    None,
    Invalid,
    Int(u32),
    String(String),
    Strings(Vec<String>),
    Binary(Vec<u8>),
}

impl RegistryProperty {
    fn into_string(self) -> Result<String, PciInfoError> {
        match self {
            Self::String(s) => Ok(s),
            v => Err(PciInfoError::ParseError(
                format!("Expected string, found '{v:?}'").into(),
            )),
        }
    }
    fn into_strings(self) -> Result<Vec<String>, PciInfoError> {
        match self {
            Self::Strings(s) => Ok(s),
            v => Err(PciInfoError::ParseError(
                format!("Expected string, found '{v:?}'").into(),
            )),
        }
    }

    fn deserialize(property_data_type: REG_VALUE_TYPE, buffer: Vec<u8>) -> RegistryProperty {
        match property_data_type {
            REG_NONE => Self::None,
            REG_SZ | REG_EXPAND_SZ => Self::from_sz(buffer),
            REG_MULTI_SZ => Self::from_msz(buffer),
            REG_BINARY => Self::Binary(vec![]),
            REG_DWORD_LITTLE_ENDIAN if buffer.len() >= 4 => {
                let bytes = [buffer[0], buffer[1], buffer[2], buffer[3]];
                Self::Int(u32::from_le_bytes(bytes))
            }
            REG_DWORD_BIG_ENDIAN if buffer.len() >= 4 => {
                let bytes = [buffer[0], buffer[1], buffer[2], buffer[3]];
                Self::Int(u32::from_be_bytes(bytes))
            }
            _ => Self::Invalid,
        }
    }

    fn from_sz(buffer: Vec<u8>) -> RegistryProperty {
        let sz = buffer
            .chunks_exact(2)
            .map(|w| u16::from_le_bytes([w[0], w[1]]))
            .take_while(|u| *u != 0)
            .filter_map(|u| char::from_u32(u as u32))
            .collect::<String>();

        Self::String(sz)
    }

    fn from_msz(buffer: Vec<u8>) -> RegistryProperty {
        let mut strings = Vec::new();

        let mut stream = buffer
            .chunks_exact(2)
            .map(|w| u16::from_le_bytes([w[0], w[1]]));

        loop {
            let s = (&mut stream)
                .take_while(|u| *u != 0)
                .filter_map(|u| char::from_u32(u as u32))
                .collect::<String>();

            if !s.is_empty() {
                strings.push(s);
            } else {
                break;
            }
        }

        Self::Strings(strings)
    }
}

unsafe fn read_device_property(
    device_info_set: setupapi::HDEVINFO,
    device_info_data: setupapi::SP_DEVINFO_DATA,
    property_id: setupapi::SETUP_DI_REGISTRY_PROPERTY,
) -> Result<RegistryProperty, PciInfoError> {
    let mut property_data_type = 0u32;
    let mut expected_size = 0u32;

    match setupapi::SetupDiGetDeviceRegistryPropertyW(
        device_info_set,
        &device_info_data,
        property_id,
        Some(&mut property_data_type),
        None,
        Some(&mut expected_size),
    ) {
        Ok(()) => (),
        Err(e) if e.code() == HRESULT::from_win32(ERROR_INSUFFICIENT_BUFFER.0) => (),
        Err(e) if e.code() == HRESULT::from_win32(ERROR_INVALID_DATA.0) => {
            return Ok(RegistryProperty::Invalid)
        }
        Err(e) => Err(e)?,
    }

    let mut buffer = vec![0u8; expected_size as usize];

    setupapi::SetupDiGetDeviceRegistryPropertyW(
        device_info_set,
        &device_info_data,
        property_id,
        Some(&mut property_data_type),
        Some(&mut buffer),
        None,
    )?;

    Ok(RegistryProperty::deserialize(
        REG_VALUE_TYPE(property_data_type),
        buffer,
    ))
}

pub(super) fn enumerate_pci() -> Result<PciInfo, PciInfoError> {
    let mut pi = PciInfo::empty();
    unsafe {
        let device_info_set = setupapi::SetupDiGetClassDevsW(
            Some(&GUID_DEVCLASS_SYSTEM as *const GUID),
            PCWSTR(ptr::null()),
            HWND(0),
            setupapi::SETUP_DI_GET_CLASS_DEVS_FLAGS(
                setupapi::DIGCF_PRESENT.0 | setupapi::DIGCF_ALLCLASSES.0,
            ),
        )?;

        let mut device_info_data = setupapi::SP_DEVINFO_DATA {
            cbSize: mem::size_of::<setupapi::SP_DEVINFO_DATA>() as u32,
            ClassGuid: GUID::default(),
            DevInst: 0,
            Reserved: 0,
        };

        let mut index = 0u32;

        while setupapi::SetupDiEnumDeviceInfo(device_info_set, index, &mut device_info_data).is_ok()
        {
            let result = enumerate_device(device_info_set, device_info_data);
            match result {
                Ok(None) => (),
                Ok(Some(d)) => pi.results.push(d.try_into_device(true).map_err(|e| {
                    PciDeviceEnumerationError::new(
                        crate::PciDeviceEnumerationErrorImpact::Device,
                        e,
                    )
                })),
                Err(e) => pi.push_error(PciDeviceEnumerationError::new(
                    crate::PciDeviceEnumerationErrorImpact::Device,
                    e,
                )),
            };

            index += 1;
        }

        let _ = setupapi::SetupDiDestroyDeviceInfoList(device_info_set);
    }

    Ok(pi)
}

unsafe fn enumerate_device(
    device_info_set: setupapi::HDEVINFO,
    device_info_data: setupapi::SP_DEVINFO_DATA,
) -> Result<Option<DeviceEntry>, PciInfoError> {
    let mut device_instance_id = vec![0u16; MAX_DEVICE_ID_LEN];
    let mut device_instance_id_len = 0u32;

    setupapi::SetupDiGetDeviceInstanceIdW(
        device_info_set,
        &device_info_data,
        Some(&mut device_instance_id),
        Some(&mut device_instance_id_len),
    )?;

    if device_instance_id.len() < 4 || &device_instance_id[0..4] != PCI_PREFIX {
        return Ok(None);
    }

    let hardware_ids = read_device_property(
        device_info_set,
        device_info_data,
        setupapi::SPDRP_HARDWAREID,
    )?
    .into_strings()?;
    let compatible_ids = read_device_property(
        device_info_set,
        device_info_data,
        setupapi::SPDRP_COMPATIBLEIDS,
    )?
    .into_strings()?;

    let mut de = DeviceEntry::default();

    for hwid in hardware_ids.iter().chain(compatible_ids.iter()) {
        if !hwid.starts_with("PCI\\") {
            continue;
        }

        let (_, hwid) = hwid.split_at(4);

        try_parse_windows_hwid(&mut de, hwid);
    }

    de.location
        .try_replace(parse_pci_location(device_info_set, device_info_data));

    Ok(Some(de))
}

unsafe fn parse_pci_location(
    device_info_set: setupapi::HDEVINFO,
    device_info_data: setupapi::SP_DEVINFO_DATA,
) -> Result<PciLocation, PciInfoError> {
    let sloc = read_device_property(
        device_info_set,
        device_info_data,
        setupapi::SPDRP_LOCATION_INFORMATION,
    )?
    .into_string()?;
    let s: String = sloc
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',')
        .collect();

    let parts = s
        .split(',')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect::<Vec<_>>();

    if parts.len() == 3 {
        PciLocation::with_bdf(parts[0], parts[1], parts[2])
    } else {
        Err(PciInfoError::ParseError(
            format!(
                "Expected 3 integer elements in pci location, found {} in '{}'",
                parts.len(),
                sloc
            )
            .into(),
        ))
    }
}
