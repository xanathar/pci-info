use std::fs;
use std::io::{self, BufRead, Read};

use crate::pci_device::PciDeviceProperties;
use crate::pci_headers::{PciCommonHeader, PciSpecializedHeader};
use crate::pci_info::PciInfo;
use crate::pci_property_result::PropertyResult;
use crate::PciBusNumber;
use crate::{
    PciDevice, PciDeviceEnumerationError, PciDeviceEnumerationErrorImpact, PciInfoError,
    PciLocation,
};

fn read_bus_directory(
    bus_dir: Result<fs::DirEntry, std::io::Error>,
    pi: &mut PciInfo,
    read_extended_headers: bool,
) -> Result<(), PciInfoError> {
    let bus_dir = bus_dir?;

    if !bus_dir.file_type()?.is_dir() {
        return Ok(());
    }

    let bus_id = parse_bus_id(bus_dir.file_name())?;
    let devices_files = fs::read_dir(bus_dir.path())?;

    for device_file in devices_files {
        if let Err(e) = read_pci_header_file(device_file, bus_id, pi, read_extended_headers) {
            pi.push_error(PciDeviceEnumerationError::new_at_bus(
                PciBusNumber::new(bus_id),
                PciDeviceEnumerationErrorImpact::Device,
                e,
            ));
        }
    }

    Ok(())
}

fn read_pci_header_file(
    device_file: Result<fs::DirEntry, std::io::Error>,
    bus_id: u8,
    pi: &mut PciInfo,
    read_extended_headers: bool,
) -> Result<(), PciInfoError> {
    let device_file = device_file?;

    if !device_file.file_type()?.is_file() {
        return Ok(());
    }

    let (slot, func) = parse_slot_and_func(device_file.file_name())?;

    let location = PciLocation::with_bdf(bus_id, slot, func);

    let mut buffer = [0; PciCommonHeader::MAX_HEADER_LEN];
    let mut f = fs::File::open(device_file.path())?;

    read_loop(&mut f, &mut buffer[0..PciCommonHeader::COMMON_HEADER_LEN])?;

    let header = PciCommonHeader::with_bytes(&buffer)?;

    let mut device = if read_extended_headers {
        let subheader_bytes = PciSpecializedHeader::length_of_subheader(header.header_type);

        let specialized = if let Some(subheader_bytes) = subheader_bytes {
            match read_loop(
                &mut f,
                &mut buffer[PciCommonHeader::COMMON_HEADER_LEN..subheader_bytes],
            ) {
                Ok(()) => PciSpecializedHeader::read_subheader(header.header_type, &buffer, true),
                Err(e) => Err(e),
            }
        } else {
            Err(PciInfoError::UnknownPciHeaderType(header.header_type))
        };

        PciDevice::from_pci_header_result(header, specialized)
    } else {
        PciDevice::from_pci_header_set(header, None)
    };

    device.properties.location.set_res(location);
    pi.push_device(device);

    Ok(())
}

fn read_loop(f: &mut fs::File, buffer: &mut [u8]) -> Result<(), PciInfoError> {
    let mut read_total = 0;

    while read_total < buffer.len() {
        let read_now = f.read(buffer)?;

        if read_now == 0 {
            return Err(PciInfoError::UnexpectedEof);
        }

        read_total += read_now;
    }

    Ok(())
}

fn parse_slot_and_func(filename: std::ffi::OsString) -> Result<(u8, u8), PciInfoError> {
    let Some(slot_func_str) = filename.to_str() else {
        return Err(PciInfoError::ParseError("bus id has invalid code".into()));
    };

    let Some((slot_str, func_str)) = slot_func_str.split_once('.') else {
        return Err(PciInfoError::ParseError(
            format!("slot id is invalid pattern, hh.h expected: '{slot_func_str}'").into(),
        ));
    };

    let slot = u8::from_str_radix(slot_str, 16).map_err(|_| {
        PciInfoError::ParseError(format!("slot id is invalid hex: '{slot_str}'").into())
    })?;

    let func = u8::from_str_radix(func_str, 16).map_err(|_| {
        PciInfoError::ParseError(format!("func id is invalid hex: '{func_str}'").into())
    })?;

    Ok((slot, func))
}

fn parse_bus_id(filename: std::ffi::OsString) -> Result<u8, PciInfoError> {
    let Some(bus_str) = filename.to_str() else {
        return Err(PciInfoError::ParseError("bus id has invalid code".into()));
    };

    u8::from_str_radix(bus_str, 16)
        .map_err(|_| PciInfoError::ParseError(format!("bus id is invalid hex: '{bus_str}'").into()))
}

struct DeviceFileEntry {
    location: PciLocation,
    vendor_id: u16,
    device_id: u16,
    irq: Option<u8>,
    kernel_driver: Option<String>,
}

impl DeviceFileEntry {
    fn new(fields: Vec<&str>) -> Result<Self, PciInfoError> {
        if fields.len() < 4 {
            return Err(PciInfoError::ParseError(
                "devices file line has not enough entries".into(),
            ));
        }

        let location =
            PciLocation::with_bdf_u16(u16::from_str_radix(fields[0], 16).map_err(|_| {
                PciInfoError::ParseError(
                    format!("bus id in devices file is invalid hex: '{}'", fields[0]).into(),
                )
            })?);
        let pci_id = u32::from_str_radix(fields[1], 16).map_err(|_| {
            PciInfoError::ParseError(
                format!(
                    "device+vendor id in devices file is invalid hex: '{}'",
                    fields[1]
                )
                .into(),
            )
        })?;
        let vendor_id = (pci_id >> 16) as u16;
        let device_id = (pci_id & 0xFFFF) as u16;
        let irq = u8::from_str_radix(fields[2], 16).ok().filter(|s| *s != 0);
        let kernel_driver = fields
            .get(17)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        Ok(Self {
            location,
            vendor_id,
            device_id,
            irq,
            kernel_driver,
        })
    }
}

fn parse_device_file() -> Result<Vec<Result<DeviceFileEntry, PciInfoError>>, PciInfoError> {
    let file = fs::File::open("/proc/bus/pci/devices")?;
    let lines = io::BufReader::new(file).lines();

    let mut entries = Vec::new();

    for line in lines {
        let line = line?;
        let fields = line.split('\t').map(|s| s.trim()).collect::<Vec<_>>();

        entries.push(DeviceFileEntry::new(fields))
    }

    Ok(entries)
}

pub(super) fn enumerate_pci(
    read_headers: bool,
    read_extended_headers: bool,
    read_device_file: bool,
) -> Result<PciInfo, PciInfoError> {
    let mut pi = PciInfo::empty();
    let bus_directories = fs::read_dir("/proc/bus/pci")?;

    let dev_file_entries = if read_device_file {
        parse_device_file()
    } else {
        Ok(Vec::new())
    };

    if read_headers {
        for bus_dir in bus_directories {
            if let Err(e) = read_bus_directory(bus_dir, &mut pi, read_extended_headers) {
                pi.push_error(PciDeviceEnumerationError::new(
                    PciDeviceEnumerationErrorImpact::Bus,
                    e,
                ));
            }
        }

        // if we *also* read the device file, let's merge the info by bus-id
        if read_device_file {
            match dev_file_entries {
                Ok(mut dev_file_entries) => {
                    for entry in dev_file_entries.drain(..) {
                        match entry {
                            Ok(mut entry) => {
                                if let Some(dev) = pi.find_device_mut(entry.location) {
                                    dev.properties.os_driver.set_val(entry.kernel_driver.take());
                                    dev.properties.os_irq.set_val(entry.irq);
                                }
                            }
                            Err(e) => pi.push_error(PciDeviceEnumerationError::new(
                                PciDeviceEnumerationErrorImpact::DeviceProperties,
                                e,
                            )),
                        }
                    }
                }
                Err(e) => pi.mutate_devices(|dev| {
                    dev.properties.os_driver.set_err(e.clone());
                    dev.properties.os_irq.set_err(e.clone());
                }),
            }
        }
    } else if read_device_file {
        let mut dev_file_entries = dev_file_entries?;

        for entry in dev_file_entries.drain(..) {
            match entry {
                Ok(d) => {
                    pi.push_device(PciDevice::new(
                        d.vendor_id,
                        d.device_id,
                        PciDeviceProperties {
                            location: PropertyResult::with_val(d.location),
                            os_irq: PropertyResult::with_val(d.irq),
                            os_driver: PropertyResult::with_val(d.kernel_driver),
                            ..Default::default()
                        },
                    ));
                }
                Err(e) => {
                    pi.push_error(PciDeviceEnumerationError::new(
                        PciDeviceEnumerationErrorImpact::Device,
                        e,
                    ));
                }
            }
        }
    } else {
        unreachable!(); // just in case
    }

    Ok(pi)
}
