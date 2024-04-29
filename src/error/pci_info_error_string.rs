use std::{borrow::Cow, fmt::Debug, fmt::Display};

// define an alias in case in the future we want
// to use Rc or Box for some platforms.
// Arc is best because it saves on cloning and is Send.
type HeapPtr<T> = std::sync::Arc<T>;

/// A string type used to optimize size and
/// various operations on the error types of this crate.
///
/// # Examples
/// This shows some conversions to `&str` and `String`
/// using different methods:
/// ```rust
/// # use pci_info::PciInfoErrorString;
/// let errs: PciInfoErrorString = "test".into();
///
/// // Convert to a String calling `to_string()`
/// let s: String = errs.to_string();
/// assert_eq!(s, "test");
///
/// // Convert to a String calling `into()`
/// let errs2 = errs.clone();
/// let s: String = errs2.into();
/// assert_eq!(s, "test");
///
/// // Convert to a &str calling `into()` on a `&PciInfoErrorString`
/// let s: &str = (&errs).into();
/// assert_eq!(s, "test");
///
/// // Convert to a &str calling `as_str()`
/// let s: &str = errs.as_str();
/// assert_eq!(s, "test");
///
/// // Use the `Display` trait
/// let s = format!("{errs}");
/// assert_eq!(s, "test");
///
/// // Use the `Debug` trait
/// let s = format!("{errs:?}");
/// assert_eq!(s, "\"test\"");
/// ```
#[derive(Clone)]
pub struct PciInfoErrorString(HeapPtr<Cow<'static, str>>);

impl From<PciInfoErrorString> for String {
    fn from(value: PciInfoErrorString) -> Self {
        value.to_string()
    }
}

impl<'a> From<&'a PciInfoErrorString> for &'a str {
    fn from(value: &'a PciInfoErrorString) -> Self {
        &value.0
    }
}

impl PciInfoErrorString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for PciInfoErrorString {
    fn from(value: &'static str) -> Self {
        Self(HeapPtr::new(value.into()))
    }
}

impl From<String> for PciInfoErrorString {
    fn from(value: String) -> Self {
        Self(HeapPtr::new(value.into()))
    }
}

impl Display for PciInfoErrorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for PciInfoErrorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::PciInfoErrorString;

    #[test]
    fn pciinfoerrorstring_staticstr() {
        let errs: PciInfoErrorString = "test".into();

        let s: String = errs.to_string();
        assert_eq!(s, "test");

        let errs2 = errs.clone();
        let s: String = errs2.into();
        assert_eq!(s, "test");

        let s: &str = (&errs).into();
        assert_eq!(s, "test");

        let s: &str = errs.as_str();
        assert_eq!(s, "test");

        let s = format!("{errs}");
        assert_eq!(s, "test");

        let s = format!("{errs:?}");
        assert_eq!(s, "\"test\"");
    }

    #[test]
    fn pciinfoerrorstring_string() {
        let errs: PciInfoErrorString = format!("test").into();

        let s: String = errs.to_string();
        assert_eq!(s, "test");

        let errs2 = errs.clone();
        let s: String = errs2.into();
        assert_eq!(s, "test");

        let s: &str = (&errs).into();
        assert_eq!(s, "test");

        let s: &str = errs.as_str();
        assert_eq!(s, "test");

        let s = format!("{errs}");
        assert_eq!(s, "test");

        let s = format!("{errs:?}");
        assert_eq!(s, "\"test\"");
    }
}
