//! This module contains enumerations describing the possible value for
//! device class codes ([`PciDeviceClass`]), device subclass codes
//! ([`PciDeviceSubclass`]) and interface function codes
//! ([`PciDeviceInterfaceFunc`]).

#[rustfmt::skip]
mod device_class;
#[rustfmt::skip]
mod device_interface_func;
#[rustfmt::skip]
mod device_subclass;

pub use device_class::PciDeviceClass;
pub use device_interface_func::PciDeviceInterfaceFunc;
pub use device_subclass::PciDeviceSubclass;
