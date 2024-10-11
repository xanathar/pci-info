// See https://man.freebsd.org/cgi/man.cgi?query=pci&sektion=4&manpath=freebsd-release-ports

use std::{ffi::CStr, fs::File, io, os::fd::AsRawFd};

use crate::{
    pci_device::PciDeviceProperties, pci_property_result::PropertyResult, PciDevice,
    PciDeviceEnumerationError, PciDeviceEnumerationErrorImpact, PciInfo, PciInfoError, PciLocation,
};

// The ioctl must be restarted multiple times in case it returns
// PCI_GETCONF_LIST_CHANGED. We restart a maximum of 5 (or whatever
// its written here) times.
const MAX_CHANGED_LOOPS: u32 = 5;

const PCIOCGETCONF: libc::c_ulong = 0xC0307005;

const MAX_NAME_LEN: usize = 16;

type PciGetConfStatus = u32;
const PCI_GETCONF_LAST_DEVICE: PciGetConfStatus = 0;
const PCI_GETCONF_LIST_CHANGED: PciGetConfStatus = 1;
const PCI_GETCONF_MORE_DEVS: PciGetConfStatus = 2;
const PCI_GETCONF_ERROR: PciGetConfStatus = 3;

#[allow(dead_code)]
#[repr(C)]
struct PciConfIo {
    pat_buf_len: u32,
    num_patterns: u32,
    patterns: *const libc::c_void,
    match_buf_len: u32,
    num_matches: u32,
    matches: *mut PciConf,
    offset: u32,
    generation: u32,
    status: PciGetConfStatus,
}

#[derive(Default)]
#[repr(C)]
struct PciSel {
    pc_domain: u32,
    pc_bus: u8,
    pc_dev: u8,
    pc_func: u8,
}

#[derive(Default)]
#[repr(C)]
struct PciConf {
    pc_sel: PciSel,
    pc_hdr: u8,
    pc_subvendor: u16,
    pc_subdevice: u16,
    pc_vendor: u16,
    pc_device: u16,
    pc_class: u8,
    pc_subclass: u8,
    pc_progif: u8,
    pc_revid: u8,
    pd_name: [u8; MAX_NAME_LEN + 1],
    pd_unit: libc::c_long,
}

pub(super) unsafe fn enumerate_devices() -> Result<PciInfo, PciInfoError> {
    let file = File::open("/dev/pci")?;
    let devpci_file = file.as_raw_fd();

    for _ in 0..MAX_CHANGED_LOOPS {
        let mut offset = 0;
        let mut pci_info = PciInfo::empty();
        let mut dev = PciConf::default();

        // not documented (afaik) but the API expects the object not to
        // move in memory, so we init it outside the inner loop
        let mut pc = PciConfIo {
            pat_buf_len: 0,
            num_patterns: 0,
            patterns: std::ptr::null(),
            match_buf_len: std::mem::size_of::<PciConf>() as u32,
            num_matches: 1,
            matches: &mut dev as *mut _,
            offset: 0,
            generation: 0,
            status: PCI_GETCONF_LAST_DEVICE,
        };

        loop {
            match libc::ioctl(devpci_file, PCIOCGETCONF, &mut pc as *mut _) {
                libc::EBADF | libc::ENOTTY | libc::EFAULT => {
                    return Err(PciInfoError::IoError(Box::new(io::ErrorKind::InvalidData)))
                }
                libc::EINVAL => {
                    return Err(PciInfoError::IoError(Box::new(io::ErrorKind::Unsupported)))
                }
                err if err < 0 => {
                    return Err(PciInfoError::IoError(Box::new(io::ErrorKind::Other)))
                }
                _ => (),
            }

            if pc.status == PCI_GETCONF_LIST_CHANGED {
                break;
            }

            if pc.status == PCI_GETCONF_ERROR {
                return Err(PciInfoError::EnumerationInterrupted(
                    "enumeration interrupted with PCI_GETCONF_ERROR".into(),
                ));
            }

            if pc.num_matches == 0 {
                if pci_info.results.is_empty() {
                    return Err(PciInfoError::EnumerationInterrupted(
                        "enumeration interrupted with no matches".into(),
                    ));
                } else {
                    pci_info.push_error(PciDeviceEnumerationError::new(
                        PciDeviceEnumerationErrorImpact::Bus,
                        PciInfoError::EnumerationInterrupted(
                            "enumeration interrupted with no matches".into(),
                        ),
                    ))
                }
            }

            if dev.pc_vendor == 0 {
                pci_info.push_error(PciDeviceEnumerationError::new(
                    PciDeviceEnumerationErrorImpact::Device,
                    PciInfoError::ValueNotFound(Some("pc_vendor".into())),
                ));
            } else if dev.pc_device == 0 {
                pci_info.push_error(PciDeviceEnumerationError::new(
                    PciDeviceEnumerationErrorImpact::Device,
                    PciInfoError::ValueNotFound(Some("pc_device".into())),
                ));
            } else {
                let (sub_v, sub_d) = if dev.pc_subvendor != 0 && dev.pc_subdevice != 0 {
                    (Some(dev.pc_subvendor), Some(dev.pc_subdevice))
                } else {
                    (None, None)
                };

                dev.pd_name[MAX_NAME_LEN] = 0;
                let name = CStr::from_bytes_until_nul(&dev.pd_name).unwrap();
                let name = name.to_string_lossy().into_owned();

                pci_info.push_device(PciDevice::new(
                    dev.pc_vendor,
                    dev.pc_device,
                    PciDeviceProperties {
                        location: PropertyResult::with_res(PciLocation::with_segment(
                            (dev.pc_sel.pc_domain & 0xFFFF) as u16,
                            dev.pc_sel.pc_bus,
                            dev.pc_sel.pc_dev,
                            dev.pc_sel.pc_func,
                        )),

                        subsystem_vendor_id: PropertyResult::with_val(sub_v),
                        subsystem_device_id: PropertyResult::with_val(sub_d),
                        revision: PropertyResult::with_val(dev.pc_revid),
                        device_class: PropertyResult::with_val(dev.pc_class),
                        device_subclass: PropertyResult::with_val(dev.pc_subclass),
                        device_iface: PropertyResult::with_val(dev.pc_progif),
                        os_driver: PropertyResult::with_val(Some(name)),
                        ..Default::default()
                    },
                ));
            }

            offset += pc.num_matches;
            pc.offset = offset;

            if pc.status != PCI_GETCONF_MORE_DEVS {
                return Ok(pci_info);
            }
        }
    }

    Err(PciInfoError::DevicesChangedTooManyTimes)
}
