// Dead code is allowed in this module as it serves multiple platforms
#![allow(dead_code)]

use crate::{PciInfoError, PciInfoPropertyError, PciLocation};
use std::{fmt, ops::Deref};

pub(crate) struct PropertyResult<T>(Result<T, PciInfoPropertyError>);

impl<T> PropertyResult<T> {
    pub fn with_val(val: T) -> Self {
        Self(Ok(val))
    }

    pub fn with_err(err: PciInfoError) -> Self {
        Self(Err(PciInfoPropertyError::Error(Box::new(err))))
    }

    pub fn with_res(res: Result<T, PciInfoError>) -> Self {
        match res {
            Ok(v) => Self::with_val(v),
            Err(e) => Self::with_err(e),
        }
    }

    pub fn set_res_cast<U: Copy + TryInto<T> + std::fmt::Display>(
        &mut self,
        res: Result<U, PciInfoError>,
    ) {
        let v = res.and_then(|v| {
            v.try_into().map_err(|_| {
                PciInfoError::ParseError(
                    format!("Value {v} cannot fit into a {}", std::any::type_name::<T>()).into(),
                )
            })
        });

        self.set_res(v)
    }

    pub fn set_res(&mut self, res: Result<T, PciInfoError>) {
        match res {
            Ok(v) => self.set_val(v),
            Err(e) => self.set_err(e),
        }
    }

    pub fn set_val(&mut self, val: T) {
        self.0 = Ok(val);
    }

    pub fn set_err(&mut self, err: PciInfoError) {
        self.0 = Err(PciInfoPropertyError::Error(Box::new(err)));
    }

    pub fn set_empty(&mut self) {
        self.0 = Err(PciInfoPropertyError::Unsupported);
    }

    pub fn as_option(&self) -> Option<&T> {
        self.0.as_ref().ok()
    }

    pub fn as_result_ref(&self) -> Result<&T, &PciInfoPropertyError> {
        self.0.as_ref()
    }
}

impl<T> Default for PropertyResult<T> {
    fn default() -> Self {
        Self(Err(PciInfoPropertyError::Unsupported))
    }
}

impl<T: FormattablePropertyResult> fmt::Debug for PropertyResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(v) => (*v).format_property(f),
            Err(PciInfoPropertyError::Unsupported) => write!(f, "n/a"),
            Err(_) => write!(f, "ERROR"),
        }
    }
}

impl<T: PartialEq> PropertyResult<T> {
    pub fn eq_val(&self, other: &T) -> bool {
        match &self.0 {
            Ok(v) => *v == *other,
            Err(_) => false,
        }
    }
}

impl<T: Copy> PropertyResult<T> {
    pub fn as_result(&self) -> Result<T, &PciInfoPropertyError> {
        self.0.as_ref().map(|v| *v)
    }
}

impl<T: Deref> PropertyResult<T> {
    pub fn as_result_deref(&self) -> Result<&T::Target, &PciInfoPropertyError> {
        self.0.as_deref()
    }
}

trait FormattablePropertyResult {
    fn format_property(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl FormattablePropertyResult for PciLocation {
    fn format_property(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FormattablePropertyResult for u16 {
    fn format_property(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04X}", self)
    }
}

impl FormattablePropertyResult for u8 {
    fn format_property(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self)
    }
}

impl FormattablePropertyResult for Option<u16> {
    fn format_property(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Some(v) => write!(f, "{:04X}", v),
            None => write!(f, "none"),
        }
    }
}
