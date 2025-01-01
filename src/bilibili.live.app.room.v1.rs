// This file is @generated by prost-build.
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetStudioListReq {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStudioListResp {
    ///
    #[prost(int64, tag = "1")]
    pub status: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub master_list: ::prost::alloc::vec::Vec<get_studio_list_resp::StudioMaster>,
}
/// Nested message and enum types in `GetStudioListResp`.
pub mod get_studio_list_resp {
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pendants {
        ///
        #[prost(message, optional, tag = "1")]
        pub frame: ::core::option::Option<pendants::Pendant>,
        ///
        #[prost(message, optional, tag = "2")]
        pub badge: ::core::option::Option<pendants::Pendant>,
    }
    /// Nested message and enum types in `Pendants`.
    pub mod pendants {
        ///
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Pendant {
            ///
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            ///
            #[prost(int64, tag = "2")]
            pub position: i64,
            ///
            #[prost(string, tag = "3")]
            pub value: ::prost::alloc::string::String,
            ///
            #[prost(string, tag = "4")]
            pub desc: ::prost::alloc::string::String,
        }
    }
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StudioMaster {
        ///
        #[prost(int64, tag = "1")]
        pub uid: i64,
        ///
        #[prost(int64, tag = "2")]
        pub room_id: i64,
        ///
        #[prost(string, tag = "3")]
        pub uname: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "4")]
        pub face: ::prost::alloc::string::String,
        ///
        #[prost(message, optional, tag = "5")]
        pub pendants: ::core::option::Option<Pendants>,
        ///
        #[prost(string, tag = "6")]
        pub tag: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "7")]
        pub tag_type: i64,
    }
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct InteractConnect {
    ///
    #[prost(enumeration = "InteractConnectType", tag = "1")]
    pub interact_connect_type: i32,
    ///
    #[prost(enumeration = "PubStatus", tag = "2")]
    pub default_pub_status: i32,
    ///
    #[prost(enumeration = "SubMode", tag = "3")]
    pub default_sub_mode: i32,
    ///
    #[prost(enumeration = "PlayMode", tag = "4")]
    pub audio_play_mode: i32,
    ///
    #[prost(enumeration = "PlayMode", tag = "5")]
    pub video_play_mode: i32,
    ///
    #[prost(int64, tag = "6")]
    pub connect_timeout: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractLayoutData {
    ///
    #[prost(int32, tag = "1")]
    pub width: i32,
    ///
    #[prost(int32, tag = "2")]
    pub height: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub default_cell: ::core::option::Option<LayoutCell>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub cells: ::prost::alloc::vec::Vec<LayoutCell>,
    ///
    #[prost(message, optional, tag = "5")]
    pub rtc_resolution: ::core::option::Option<RtcResolution>,
    ///
    #[prost(int32, tag = "6")]
    pub best_area_show_pos: i32,
    ///
    #[prost(message, optional, tag = "7")]
    pub rtc_resolution_simulcast: ::core::option::Option<RtcResolution>,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractMode {
    ///
    #[prost(enumeration = "InteractModeType", tag = "1")]
    pub interact_mode_type: i32,
    ///
    #[prost(int32, repeated, tag = "2")]
    pub join_types: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(int64, tag = "3")]
    pub invite_timeout: i64,
    ///
    #[prost(int64, tag = "4")]
    pub apply_timeout: i64,
    ///
    #[prost(enumeration = "InteractPositionMode", tag = "5")]
    pub position_mode: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractSceneConfig {
    ///
    #[prost(string, tag = "1")]
    pub interact_scene_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub interact_template: ::core::option::Option<InteractTemplate>,
    ///
    #[prost(message, optional, tag = "3")]
    pub interact_mode: ::core::option::Option<InteractMode>,
    ///
    #[prost(message, optional, tag = "4")]
    pub interact_connect: ::core::option::Option<InteractConnect>,
    ///
    #[prost(int64, tag = "5")]
    pub interact_max_users: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractTemplate {
    ///
    #[prost(string, tag = "1")]
    pub template_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub is_variable_layout: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub layout_list: ::prost::alloc::vec::Vec<interact_template::Layout>,
    ///
    #[prost(bool, tag = "4")]
    pub show_interact_ui: bool,
    ///
    #[prost(string, tag = "5")]
    pub layout_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub layout_data: ::core::option::Option<InteractLayoutData>,
}
/// Nested message and enum types in `InteractTemplate`.
pub mod interact_template {
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Layout {
        ///
        #[prost(string, tag = "1")]
        pub layout_id: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "2")]
        pub video_size: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "3")]
        pub max_users: i64,
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayoutCell {
    ///
    #[prost(double, tag = "1")]
    pub x: f64,
    ///
    #[prost(double, tag = "2")]
    pub y: f64,
    ///
    #[prost(double, tag = "3")]
    pub width: f64,
    ///
    #[prost(double, tag = "4")]
    pub height: f64,
    ///
    #[prost(int32, tag = "5")]
    pub z_index: i32,
    ///
    #[prost(int64, tag = "6")]
    pub position: i64,
    ///
    #[prost(enumeration = "LayoutCellOpen", tag = "7")]
    pub default_open: i32,
    ///
    #[prost(int32, tag = "8")]
    pub mobile_font_size: i32,
    ///
    #[prost(int32, tag = "9")]
    pub mobile_avatar_size: i32,
    ///
    #[prost(int32, tag = "10")]
    pub pc_web_font_size: i32,
    ///
    #[prost(int32, tag = "11")]
    pub pc_web_avatar_size: i32,
    ///
    #[prost(int32, tag = "12")]
    pub can_zoom: i32,
    ///
    #[prost(int32, tag = "13")]
    pub video_index: i32,
    ///
    #[prost(string, tag = "14")]
    pub position_text: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Members {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(string, tag = "2")]
    pub uname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub position: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub stream_control: ::core::option::Option<UserStreamControl>,
    ///
    #[prost(int64, tag = "6")]
    pub join_time: i64,
    ///
    #[prost(string, tag = "7")]
    pub link_id: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "Gender", tag = "8")]
    pub gender: i32,
    ///
    #[prost(int64, tag = "9")]
    pub room_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiConnInfo {
    ///
    #[prost(message, repeated, tag = "1")]
    pub scores: ::prost::alloc::vec::Vec<multi_conn_info::Score>,
    ///
    #[prost(int64, tag = "2")]
    pub room_owner: i64,
    ///
    #[prost(int64, tag = "3")]
    pub show_score: i64,
}
/// Nested message and enum types in `MultiConnInfo`.
pub mod multi_conn_info {
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Score {
        ///
        #[prost(int64, tag = "1")]
        pub uid: i64,
        ///
        #[prost(int64, tag = "2")]
        pub price: i64,
        ///
        #[prost(string, tag = "3")]
        pub price_text: ::prost::alloc::string::String,
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiConnInfoReq {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub anchor_uid: i64,
    ///
    #[prost(string, tag = "3")]
    pub biz_session_id: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiConnInfoResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<multi_conn_info_resp::Info>,
    ///
    #[prost(int64, tag = "2")]
    pub invoking_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub version: i64,
    ///
    #[prost(string, tag = "4")]
    pub layout: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub room_status: i64,
    ///
    #[prost(string, tag = "6")]
    pub biz_session_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MultiConnInfoResp`.
pub mod multi_conn_info_resp {
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Info {
        ///
        #[prost(int64, tag = "1")]
        pub uid: i64,
        ///
        #[prost(int64, tag = "2")]
        pub room_id: i64,
        ///
        #[prost(string, tag = "3")]
        pub avatar: ::prost::alloc::string::String,
        ///
        #[prost(string, tag = "4")]
        pub nickname: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "5")]
        pub price: i64,
        ///
        #[prost(string, tag = "6")]
        pub price_text: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "7")]
        pub gender: i64,
    }
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operator {
    ///
    #[prost(int64, tag = "1")]
    pub anchor_uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub live_key: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub uid: i64,
    ///
    #[prost(string, tag = "5")]
    pub platform: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub build: i64,
    ///
    #[prost(string, tag = "8")]
    pub ip: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub mobi_app: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RtcResolution {
    ///
    #[prost(int32, tag = "1")]
    pub vertical_width: i32,
    ///
    #[prost(int32, tag = "2")]
    pub vertical_height: i32,
    ///
    #[prost(int32, tag = "3")]
    pub horizontal_width: i32,
    ///
    #[prost(int32, tag = "4")]
    pub horizontal_height: i32,
    ///
    #[prost(int64, tag = "5")]
    pub code_rate_init: i64,
    ///
    #[prost(int64, tag = "6")]
    pub code_rate_min: i64,
    ///
    #[prost(int64, tag = "7")]
    pub code_rate_max: i64,
    ///
    #[prost(double, tag = "8")]
    pub scale_down_ratio: f64,
    ///
    #[prost(int64, tag = "9")]
    pub small_bitrate_weight: i64,
    ///
    #[prost(int64, tag = "10")]
    pub big_bitrate_weight: i64,
    ///
    #[prost(int64, tag = "11")]
    pub small_max_fps: i64,
    ///
    #[prost(int64, tag = "12")]
    pub big_max_fps: i64,
    ///
    #[prost(bool, tag = "13")]
    pub small_bitrate_active: bool,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversalInfoReq {
    ///
    #[prost(string, tag = "1")]
    pub biz_session_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub anchor_uid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub room_id: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversalInfoResp {
    ///
    #[prost(string, tag = "1")]
    pub biz_session_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub interact_channel_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub interact_mode: ::core::option::Option<InteractMode>,
    ///
    #[prost(message, optional, tag = "4")]
    pub interact_template: ::core::option::Option<InteractTemplate>,
    ///
    #[prost(enumeration = "InteractConnectType", tag = "5")]
    pub interact_connect_type: i32,
    ///
    #[prost(int64, tag = "6")]
    pub interact_max_users: i64,
    ///
    #[prost(message, repeated, tag = "7")]
    pub members: ::prost::alloc::vec::Vec<Members>,
    ///
    #[prost(int64, tag = "8")]
    pub version: i64,
    ///
    #[prost(enumeration = "SessionStatus", tag = "9")]
    pub session_status: i32,
    ///
    #[prost(message, optional, tag = "10")]
    pub multi_conn_info: ::core::option::Option<MultiConnInfo>,
    ///
    #[prost(string, tag = "11")]
    pub business_label: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "12")]
    pub invoking_time: i64,
    ///
    #[prost(int64, tag = "13")]
    pub members_version: i64,
    ///
    #[prost(enumeration = "RoomStatus", tag = "14")]
    pub room_status: i32,
    ///
    #[prost(int64, tag = "15")]
    pub system_time_unix: i64,
    ///
    #[prost(int64, tag = "16")]
    pub room_owner: i64,
    ///
    #[prost(string, tag = "17")]
    pub session_start_at: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "18")]
    pub session_start_at_ts: i64,
    ///
    #[prost(string, tag = "19")]
    pub room_start_at: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "20")]
    pub room_start_at_ts: i64,
    ///
    #[prost(string, tag = "21")]
    pub trace_id: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStreamControl {
    ///
    #[prost(message, optional, tag = "1")]
    pub audio_control: ::core::option::Option<
        user_stream_control::UserStreamControlItem,
    >,
    ///
    #[prost(message, optional, tag = "2")]
    pub video_control: ::core::option::Option<
        user_stream_control::UserStreamControlItem,
    >,
    ///
    #[prost(message, optional, tag = "3")]
    pub pub_sub_control: ::core::option::Option<user_stream_control::PubSubControl>,
}
/// Nested message and enum types in `UserStreamControl`.
pub mod user_stream_control {
    ///
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ExplicitSubMode {
        ///
        #[prost(int64, tag = "1")]
        pub uid: i64,
        ///
        #[prost(enumeration = "super::SubMode", tag = "2")]
        pub sub_mode: i32,
    }
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PubSubControl {
        ///
        #[prost(enumeration = "super::PubStatus", tag = "1")]
        pub pub_status: i32,
        ///
        #[prost(message, repeated, tag = "2")]
        pub explicit_sub_users: ::prost::alloc::vec::Vec<ExplicitSubMode>,
        ///
        #[prost(int64, tag = "3")]
        pub version: i64,
        ///
        #[prost(message, optional, tag = "4")]
        pub rtc_resolution: ::core::option::Option<super::RtcResolution>,
        ///
        #[prost(map = "int64, int32", tag = "5")]
        pub sub_video_index: ::std::collections::HashMap<i64, i32>,
        ///
        #[prost(message, optional, tag = "6")]
        pub rtc_resolution_simulcast: ::core::option::Option<super::RtcResolution>,
    }
    ///
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserStreamControlItem {
        ///
        #[prost(enumeration = "super::MuteLocalMode", tag = "1")]
        pub mute_local_mode: i32,
        ///
        #[prost(int64, repeated, tag = "2")]
        pub mute_remote_uids: ::prost::alloc::vec::Vec<i64>,
        ///
        #[prost(int64, repeated, tag = "3")]
        pub receive_uids: ::prost::alloc::vec::Vec<i64>,
        ///
        #[prost(int64, tag = "4")]
        pub version: i64,
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gender {
    ///
    Woman = 0,
    ///
    Man = 1,
}
impl Gender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Woman => "Gender_Woman",
            Self::Man => "Gender_Man",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Gender_Woman" => Some(Self::Woman),
            "Gender_Man" => Some(Self::Man),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InteractConnectType {
    ///
    All = 0,
    ///
    OnlyVoice = 1,
    ///
    OnlyVideo = 2,
}
impl InteractConnectType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::All => "InteractConnectTypeAll",
            Self::OnlyVoice => "InteractConnectTypeOnlyVoice",
            Self::OnlyVideo => "InteractConnectTypeOnlyVideo",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InteractConnectTypeAll" => Some(Self::All),
            "InteractConnectTypeOnlyVoice" => Some(Self::OnlyVoice),
            "InteractConnectTypeOnlyVideo" => Some(Self::OnlyVideo),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InteractJoinType {
    ///
    Invalid = 0,
    ///
    Invitation = 1,
    ///
    Apply = 2,
    ///
    ChannelLink = 3,
    ///
    PullUp = 4,
}
impl InteractJoinType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "InteractJoinTypeInvalid",
            Self::Invitation => "InteractJoinTypeInvitation",
            Self::Apply => "InteractJoinTypeApply",
            Self::ChannelLink => "InteractJoinTypeChannelLink",
            Self::PullUp => "InteractJoinTypePullUp",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InteractJoinTypeInvalid" => Some(Self::Invalid),
            "InteractJoinTypeInvitation" => Some(Self::Invitation),
            "InteractJoinTypeApply" => Some(Self::Apply),
            "InteractJoinTypeChannelLink" => Some(Self::ChannelLink),
            "InteractJoinTypePullUp" => Some(Self::PullUp),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InteractModeType {
    ///
    InteractModeB2b = 0,
    ///
    InteractModeB2c = 1,
}
impl InteractModeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::InteractModeB2b => "InteractModeB2B",
            Self::InteractModeB2c => "InteractModeB2C",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InteractModeB2B" => Some(Self::InteractModeB2b),
            "InteractModeB2C" => Some(Self::InteractModeB2c),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InteractPositionMode {
    ///
    Default = 0,
    ///
    Chronological = 1,
    ///
    Custom = 2,
}
impl InteractPositionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Default => "InteractPositionModeDefault",
            Self::Chronological => "InteractPositionModeChronological",
            Self::Custom => "InteractPositionModeCustom",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InteractPositionModeDefault" => Some(Self::Default),
            "InteractPositionModeChronological" => Some(Self::Chronological),
            "InteractPositionModeCustom" => Some(Self::Custom),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LayoutCellOpen {
    ///
    Invalid = 0,
    ///
    Visible = 1,
    ///
    Invisible = 2,
}
impl LayoutCellOpen {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Invalid => "LayoutCellOpenInvalid",
            Self::Visible => "LayoutCellOpenVisible",
            Self::Invisible => "LayoutCellOpenInvisible",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LayoutCellOpenInvalid" => Some(Self::Invalid),
            "LayoutCellOpenVisible" => Some(Self::Visible),
            "LayoutCellOpenInvisible" => Some(Self::Invisible),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MuteLocalMode {
    ///
    MuteModeNone = 0,
    ///
    MuteModePacket = 1,
    ///
    MuteModeCapture = 2,
}
impl MuteLocalMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::MuteModeNone => "mute_mode_none",
            Self::MuteModePacket => "mute_mode_packet",
            Self::MuteModeCapture => "mute_mode_capture",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "mute_mode_none" => Some(Self::MuteModeNone),
            "mute_mode_packet" => Some(Self::MuteModePacket),
            "mute_mode_capture" => Some(Self::MuteModeCapture),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayMode {
    ///
    Default = 0,
    ///
    Explicit = 1,
}
impl PlayMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Default => "play_mode_default",
            Self::Explicit => "play_mode_explicit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "play_mode_default" => Some(Self::Default),
            "play_mode_explicit" => Some(Self::Explicit),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PubStatus {
    ///
    None = 0,
    ///
    Audio = 1,
    ///
    Video = 2,
    ///
    All = 3,
}
impl PubStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "pub_status_none",
            Self::Audio => "pub_status_audio",
            Self::Video => "pub_status_video",
            Self::All => "pub_status_all",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "pub_status_none" => Some(Self::None),
            "pub_status_audio" => Some(Self::Audio),
            "pub_status_video" => Some(Self::Video),
            "pub_status_all" => Some(Self::All),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomStatus {
    ///
    NotStart = 0,
    ///
    Started = 1,
    ///
    Ended = 2,
}
impl RoomStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::NotStart => "RoomStatusNotStart",
            Self::Started => "RoomStatusStarted",
            Self::Ended => "RoomStatusEnded",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RoomStatusNotStart" => Some(Self::NotStart),
            "RoomStatusStarted" => Some(Self::Started),
            "RoomStatusEnded" => Some(Self::Ended),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionStatus {
    ///
    NotStart = 0,
    ///
    Started = 1,
    ///
    Ended = 2,
}
impl SessionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::NotStart => "SessionStatusNotStart",
            Self::Started => "SessionStatusStarted",
            Self::Ended => "SessionStatusEnded",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SessionStatusNotStart" => Some(Self::NotStart),
            "SessionStatusStarted" => Some(Self::Started),
            "SessionStatusEnded" => Some(Self::Ended),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubMode {
    ///
    None = 0,
    ///
    Audio = 1,
    ///
    Video = 2,
    ///
    All = 3,
}
impl SubMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::None => "sub_mode_none",
            Self::Audio => "sub_mode_audio",
            Self::Video => "sub_mode_video",
            Self::All => "sub_mode_all",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "sub_mode_none" => Some(Self::None),
            "sub_mode_audio" => Some(Self::Audio),
            "sub_mode_video" => Some(Self::Video),
            "sub_mode_all" => Some(Self::All),
            _ => None,
        }
    }
}