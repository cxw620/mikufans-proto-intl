// This file is @generated by prost-build.
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BattleInfo {
    ///
    #[prost(int64, tag = "1")]
    pub pk_i_d: i64,
    ///
    #[prost(int64, tag = "2")]
    pub status: i64,
    ///
    #[prost(int64, tag = "3")]
    pub battle_type: i64,
    ///
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub init_info: ::core::option::Option<battle_info::RoomInfo>,
    ///
    #[prost(message, optional, tag = "6")]
    pub match_info: ::core::option::Option<battle_info::RoomInfo>,
}
/// Nested message and enum types in `BattleInfo`.
pub mod battle_info {
    ///
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RoomInfo {
        ///
        #[prost(int64, tag = "1")]
        pub room_i_d: i64,
        ///
        #[prost(int64, tag = "2")]
        pub votes: i64,
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveBaseOperateEvent {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub total_price: i64,
    ///
    #[prost(int64, tag = "3")]
    pub ts: i64,
    ///
    #[prost(int64, tag = "4")]
    pub position: i64,
    ///
    #[prost(int64, tag = "5")]
    pub version: i64,
    ///
    #[prost(string, tag = "6")]
    pub total_price_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub pk_total_price: i64,
    ///
    #[prost(string, tag = "8")]
    pub pk_total_price_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub pk_group_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub pk_group_total_price: i64,
    ///
    #[prost(string, tag = "11")]
    pub pk_group_total_price_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "12")]
    pub hat: ::core::option::Option<LiveMultiChatHatLevel>,
    ///
    #[prost(int64, tag = "13")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "14")]
    pub pk_i_d: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "15")]
    pub battle_info: ::core::option::Option<BattleInfo>,
    ///
    #[prost(bool, tag = "16")]
    pub is_pei_pei: bool,
    ///
    #[prost(int64, tag = "17")]
    pub pos_room_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveMultiChatHatLevel {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub level: i64,
    ///
    #[prost(string, tag = "6")]
    pub background_color_start: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub background_color_end: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub animation_webp: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub animation_svga: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub price: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveMultiVoiceApplicationUserModel {
    ///
    #[prost(int64, tag = "1")]
    pub count: i64,
    ///
    #[prost(int64, tag = "2")]
    pub u_i_d: i64,
    ///
    #[prost(int64, tag = "3")]
    pub room_id: i64,
    ///
    #[prost(enumeration = "LiveMultiVoiceRole", tag = "4")]
    pub role: i32,
    ///
    #[prost(int64, tag = "6")]
    pub anchor_u_i_d: i64,
    ///
    #[prost(int64, tag = "7")]
    pub operate_u_i_d: i64,
    ///
    #[prost(int64, tag = "8")]
    pub want_position: i64,
    ///
    #[prost(int64, tag = "9")]
    pub event: i64,
    ///
    #[prost(string, tag = "10")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub channel: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub trace_id: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveMultiVoiceInvitaionEvent {
    ///
    #[prost(int64, tag = "1")]
    pub initiator: i64,
    ///
    #[prost(int64, tag = "2")]
    pub invited_uid: i64,
    ///
    #[prost(enumeration = "LiveInteractBusinessType", tag = "3")]
    pub business_type: i32,
    ///
    #[prost(string, tag = "4")]
    pub ch_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub channel_info: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub cdn: i32,
    ///
    #[prost(int64, tag = "7")]
    pub interact_id: i64,
    ///
    #[prost(int64, tag = "8")]
    pub link_id: i64,
    ///
    #[prost(int64, tag = "9")]
    pub ts: i64,
    ///
    #[prost(string, tag = "10")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "LiveInteractOperationType", tag = "11")]
    pub operate_type: i32,
    ///
    #[prost(string, tag = "12")]
    pub inner_extra: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub operation_uname: ::prost::alloc::string::String,
    ///
    #[prost(int64, repeated, tag = "14")]
    pub exist_uids: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(string, tag = "15")]
    pub trace_id: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveMultiVoiceJoinEvent {
    ///
    #[prost(string, tag = "1")]
    pub ch_id: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "LiveInteractBusinessType", tag = "2")]
    pub business_type: i32,
    ///
    #[prost(string, tag = "3")]
    pub inner_extra: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub initiator: i64,
    ///
    #[prost(int64, tag = "6")]
    pub invited_uid: i64,
    ///
    #[prost(int32, tag = "7")]
    pub cdn: i32,
    ///
    #[prost(string, tag = "8")]
    pub channel_info: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub interact_id: i64,
    ///
    #[prost(int64, tag = "10")]
    pub link_id: i64,
    ///
    #[prost(int64, tag = "11")]
    pub ts: i64,
    ///
    #[prost(int64, repeated, tag = "12")]
    pub exist_uids: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(string, tag = "13")]
    pub trace_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub biz_session_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub join_token: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "16")]
    pub reconnect: bool,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiveInteractBusinessType {
    ///
    Node = 0,
    ///
    MultiVoice = 10,
}
impl LiveInteractBusinessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Node => "LiveInteractBusinessTypeNode",
            Self::MultiVoice => "LiveInteractBusinessTypeMultiVoice",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LiveInteractBusinessTypeNode" => Some(Self::Node),
            "LiveInteractBusinessTypeMultiVoice" => Some(Self::MultiVoice),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiveInteractOperationType {
    ///
    Invalid = 0,
    ///
    Accept = 1,
    ///
    Reject = 2,
    ///
    Cancel = 3,
    ///
    NoReply = 4,
    ///
    Invited = 5,
}
impl LiveInteractOperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "LiveInteractOperationTypeInvalid",
            Self::Accept => "LiveInteractOperationTypeAccept",
            Self::Reject => "LiveInteractOperationTypeReject",
            Self::Cancel => "LiveInteractOperationTypeCancel",
            Self::NoReply => "LiveInteractOperationTypeNoReply",
            Self::Invited => "LiveInteractOperationTypeInvited",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LiveInteractOperationTypeInvalid" => Some(Self::Invalid),
            "LiveInteractOperationTypeAccept" => Some(Self::Accept),
            "LiveInteractOperationTypeReject" => Some(Self::Reject),
            "LiveInteractOperationTypeCancel" => Some(Self::Cancel),
            "LiveInteractOperationTypeNoReply" => Some(Self::NoReply),
            "LiveInteractOperationTypeInvited" => Some(Self::Invited),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LiveMultiVoiceRole {
    ///
    Common = 0,
    ///
    Manager = 1,
    ///
    Anchor = 2,
}
impl LiveMultiVoiceRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Common => "LiveMultiVoiceRoleCommon",
            Self::Manager => "LiveMultiVoiceRoleManager",
            Self::Anchor => "LiveMultiVoiceRoleAnchor",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LiveMultiVoiceRoleCommon" => Some(Self::Common),
            "LiveMultiVoiceRoleManager" => Some(Self::Manager),
            "LiveMultiVoiceRoleAnchor" => Some(Self::Anchor),
            _ => None,
        }
    }
}