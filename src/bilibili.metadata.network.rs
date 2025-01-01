// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    ///
    #[prost(enumeration = "NetworkType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(enumeration = "TfType", tag = "2")]
    pub tf: i32,
    ///
    #[prost(string, tag = "3")]
    pub oid: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NetworkType {
    ///
    NtUnknown = 0,
    ///
    Wifi = 1,
    ///
    Cellular = 2,
    ///
    Offline = 3,
    ///
    Othernet = 4,
    ///
    Ethernet = 5,
}
impl NetworkType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::NtUnknown => "NT_UNKNOWN",
            Self::Wifi => "WIFI",
            Self::Cellular => "CELLULAR",
            Self::Offline => "OFFLINE",
            Self::Othernet => "OTHERNET",
            Self::Ethernet => "ETHERNET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NT_UNKNOWN" => Some(Self::NtUnknown),
            "WIFI" => Some(Self::Wifi),
            "CELLULAR" => Some(Self::Cellular),
            "OFFLINE" => Some(Self::Offline),
            "OTHERNET" => Some(Self::Othernet),
            "ETHERNET" => Some(Self::Ethernet),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TfType {
    ///
    TfUnknown = 0,
    ///
    UCard = 1,
    ///
    UPkg = 2,
    ///
    CCard = 3,
    ///
    CPkg = 4,
    ///
    TCard = 5,
    ///
    TPkg = 6,
}
impl TfType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::TfUnknown => "TF_UNKNOWN",
            Self::UCard => "U_CARD",
            Self::UPkg => "U_PKG",
            Self::CCard => "C_CARD",
            Self::CPkg => "C_PKG",
            Self::TCard => "T_CARD",
            Self::TPkg => "T_PKG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TF_UNKNOWN" => Some(Self::TfUnknown),
            "U_CARD" => Some(Self::UCard),
            "U_PKG" => Some(Self::UPkg),
            "C_CARD" => Some(Self::CCard),
            "C_PKG" => Some(Self::CPkg),
            "T_CARD" => Some(Self::TCard),
            "T_PKG" => Some(Self::TPkg),
            _ => None,
        }
    }
}
