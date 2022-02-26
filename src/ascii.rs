use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use zbus::zvariant::{OwnedValue, Type, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct AsciiString(Vec<u8>);

impl TryFrom<OwnedValue> for AsciiString {
    type Error = zbus::zvariant::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        Vec::<u8>::try_from(value).map(|val| Self(val))
    }
}

impl<'a> TryFrom<Value<'a>> for AsciiString {
    type Error = zbus::zvariant::Error;

    fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
        Vec::<u8>::try_from(value).map(|val| Self(val))
    }
}

impl Display for AsciiString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = CStr::from_bytes_with_nul(&self.0)
            .unwrap()
            .to_str()
            .unwrap();
        std::fmt::Display::fmt(str, f)
    }
}

impl Into<String> for AsciiString {
    fn into(self) -> String {
        CString::from_vec_with_nul(self.0)
            .unwrap()
            .into_string()
            .unwrap()
    }
}

pub trait IntoStringVec {
    fn into_vec(self) -> Vec<String>;
}

impl IntoStringVec for Vec<AsciiString> {
    fn into_vec(self) -> Vec<String> {
        self.into_iter().map(|s| s.into()).collect()
    }
}
