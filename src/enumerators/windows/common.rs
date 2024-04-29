use crate::pci_property_result::PropertyResult;
use crate::{pci_device::PciDeviceProperties, PciDevice, PciInfoError, PciLocation};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub(super) enum ConflictOption<T: Copy + Hash + Eq + PartialEq + Debug> {
    Empty,
    Value(T),
    Error(PciInfoError),
    Conflict(HashSet<T>),
}

impl<T: Copy + Hash + Eq + PartialEq + Debug> Default for ConflictOption<T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T: Copy + Hash + Eq + PartialEq + Debug> ConflictOption<T> {
    pub fn replace(&mut self, val: T) {
        *self = match self {
            ConflictOption::Empty => ConflictOption::Value(val),
            ConflictOption::Value(old) if *old == val => return,
            ConflictOption::Value(old) => {
                let mut hs = HashSet::new();
                hs.insert(*old);
                hs.insert(val);
                ConflictOption::Conflict(hs)
            }
            ConflictOption::Error(_) => ConflictOption::Value(val),
            ConflictOption::Conflict(hs) => {
                hs.insert(val);
                return;
            }
        }
    }

    pub fn try_replace(&mut self, val: Result<T, PciInfoError>) {
        match val {
            Ok(val) => self.replace(val),
            Err(e) if self.is_empty() => *self = ConflictOption::Error(e),
            Err(_) => (),
        }
    }

    pub fn into_optvalue(self) -> Result<Option<T>, PciInfoError> {
        match self {
            Self::Empty => Ok(None),
            Self::Value(v) => Ok(Some(v)),
            Self::Error(e) => Err(e),
            Self::Conflict(v) => Err(PciInfoError::InconsistentValue(
                v.iter().map(|i| format!("{i:?}")).collect(),
            )),
        }
    }

    pub fn into_value(self) -> Result<T, PciInfoError> {
        match self {
            Self::Empty => Err(PciInfoError::ValueNotFound(None)),
            Self::Value(v) => Ok(v),
            Self::Error(e) => Err(e),
            Self::Conflict(v) => Err(PciInfoError::InconsistentValue(
                v.iter().map(|i| format!("{i:?}")).collect(),
            )),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, ConflictOption::Empty)
    }
}

#[derive(Default)]
pub(super) struct DeviceEntry {
    pub(super) vendor_id: ConflictOption<u16>,
    pub(super) device_id: ConflictOption<u16>,
    pub(super) revision: ConflictOption<u8>,
    pub(super) subsystem: ConflictOption<u32>,
    pub(super) class_code_long: ConflictOption<u32>,
    pub(super) class_code_short: ConflictOption<u32>,
    pub(super) location: ConflictOption<PciLocation>,
}

impl DeviceEntry {
    pub fn try_into_device(self, location_mandatory: bool) -> Result<PciDevice, PciInfoError> {
        let vendor_id = self.vendor_id.into_value()?;
        let device_id = self.device_id.into_value()?;

        let class_code_long = self.class_code_long.into_value();
        let class_code_short = self.class_code_short.into_value();

        let (c, s, f) = match (class_code_long, class_code_short) {
            (Ok(class_code), _) => (
                Ok(((class_code & 0xFF0000) >> 16) as u8),
                Ok(((class_code & 0xFF00) >> 8) as u8),
                Ok((class_code & 0xFF) as u8),
            ),
            (_, Ok(class_code)) => (
                Ok(((class_code & 0xFF00) >> 8) as u8),
                Ok((class_code & 0xFF) as u8),
                Err(PciInfoError::ValueNotFound(None)),
            ),
            (Err(e), _) => (Err(e.clone()), Err(e.clone()), Err(e)),
        };

        let (sub_v, sub_d) = match self.subsystem.into_optvalue() {
            Ok(None) => (Ok(None), Ok(None)),
            Ok(Some(v)) => (Ok(Some((v >> 16) as u16)), Ok(Some((v & 0xFFFF) as u16))),
            Err(e) => (Err(e.clone()), Err(e)),
        };

        let location = match self.location {
            ConflictOption::Empty if !location_mandatory => PropertyResult::default(),
            _ => PropertyResult::with_res(self.location.into_value()),
        };

        Ok(PciDevice::new(
            vendor_id,
            device_id,
            PciDeviceProperties {
                location,
                revision: PropertyResult::with_res(self.revision.into_value()),
                device_class: PropertyResult::with_res(c),
                device_subclass: PropertyResult::with_res(s),
                device_iface: PropertyResult::with_res(f),
                subsystem_vendor_id: PropertyResult::with_res(sub_v),
                subsystem_device_id: PropertyResult::with_res(sub_d),
                ..Default::default()
            },
        ))
    }
}

pub(super) fn try_parse_windows_hwid(de: &mut DeviceEntry, hwid: &str) {
    for id in hwid.split('&') {
        let Some((key, value)) = id.split_once('_') else {
            continue;
        };

        match key {
            "VEN" => de.vendor_id.try_replace(parse_hex_u16(value)),
            "DEV" => de.device_id.try_replace(parse_hex_u16(value)),
            "SUBSYS" => de.subsystem.try_replace(parse_hex_u32(value)),
            "REV" => de.revision.try_replace(parse_hex_u8(value)),
            "CC" if value.len() <= 4 => de.class_code_short.try_replace(parse_hex_u32(value)),
            "CC" => de.class_code_long.try_replace(parse_hex_u32(value)),
            _ => (),
        }
    }
}

fn parse_hex_u8(val: &str) -> Result<u8, PciInfoError> {
    u8::from_str_radix(val, 16).map_err(|_| {
        PciInfoError::ParseError(format!("attempted to parse invalid hex: '{val}'").into())
    })
}

fn parse_hex_u16(val: &str) -> Result<u16, PciInfoError> {
    u16::from_str_radix(val, 16).map_err(|_| {
        PciInfoError::ParseError(format!("attempted to parse invalid hex: '{val}'").into())
    })
}

fn parse_hex_u32(val: &str) -> Result<u32, PciInfoError> {
    u32::from_str_radix(val, 16).map_err(|_| {
        PciInfoError::ParseError(format!("attempted to parse invalid hex: '{val}'").into())
    })
}
