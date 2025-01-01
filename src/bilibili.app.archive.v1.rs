// This file is @generated by prost-build.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arc {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub videos: i64,
    ///
    #[prost(int32, tag = "3")]
    pub type_id: i32,
    ///
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub copyright: i32,
    ///
    #[prost(string, tag = "6")]
    pub pic: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub pubdate: i64,
    ///
    #[prost(int64, tag = "9")]
    pub ctime: i64,
    ///
    #[prost(string, tag = "10")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub state: i32,
    ///
    #[prost(int32, tag = "12")]
    pub access: i32,
    ///
    #[prost(int32, tag = "13")]
    pub attribute: i32,
    ///
    #[prost(string, tag = "14")]
    pub tag: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "15")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int64, tag = "16")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "17")]
    pub mission_id: i64,
    ///
    #[prost(int64, tag = "18")]
    pub order_id: i64,
    ///
    #[prost(string, tag = "19")]
    pub redirect_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "20")]
    pub forward: i64,
    ///
    #[prost(message, optional, tag = "21")]
    pub rights: ::core::option::Option<Rights>,
    ///
    #[prost(message, optional, tag = "22")]
    pub author: ::core::option::Option<Author>,
    ///
    #[prost(message, optional, tag = "23")]
    pub stat: ::core::option::Option<Stat>,
    ///
    #[prost(string, tag = "24")]
    pub report_result: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "25")]
    pub dynamic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "26")]
    pub first_cid: i64,
    ///
    #[prost(message, optional, tag = "27")]
    pub dimension: ::core::option::Option<Dimension>,
    ///
    #[prost(message, repeated, tag = "28")]
    pub staff_info: ::prost::alloc::vec::Vec<StaffInfo>,
    ///
    #[prost(int64, tag = "29")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "30")]
    pub attribute_v2: i64,
    ///
    #[prost(message, optional, tag = "31")]
    pub season_theme: ::core::option::Option<SeasonTheme>,
    ///
    #[prost(string, tag = "40")]
    pub short_link_v2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "41")]
    pub up_from_v2: i32,
    ///
    #[prost(string, tag = "42")]
    pub first_frame: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Author {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Dimension {
    ///
    #[prost(int64, tag = "1")]
    pub width: i64,
    ///
    #[prost(int64, tag = "2")]
    pub height: i64,
    ///
    #[prost(int64, tag = "3")]
    pub rotate: i64,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    ///
    #[prost(int64, tag = "1")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub page: i32,
    ///
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub part: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub duration: i64,
    ///
    #[prost(string, tag = "6")]
    pub vid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub web_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub dimension: ::core::option::Option<Dimension>,
    ///
    #[prost(string, tag = "10")]
    pub first_frame: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Rights {
    ///
    #[prost(int32, tag = "1")]
    pub bp: i32,
    ///
    #[prost(int32, tag = "2")]
    pub elec: i32,
    ///
    #[prost(int32, tag = "3")]
    pub download: i32,
    ///
    #[prost(int32, tag = "4")]
    pub movie: i32,
    ///
    #[prost(int32, tag = "5")]
    pub pay: i32,
    ///
    #[prost(int32, tag = "6")]
    pub hd5: i32,
    ///
    #[prost(int32, tag = "7")]
    pub no_reprint: i32,
    ///
    #[prost(int32, tag = "8")]
    pub autoplay: i32,
    ///
    #[prost(int32, tag = "9")]
    pub ugc_pay: i32,
    ///
    #[prost(int32, tag = "10")]
    pub is_cooperation: i32,
    ///
    #[prost(int32, tag = "11")]
    pub ugc_pay_preview: i32,
    ///
    #[prost(int32, tag = "12")]
    pub no_background: i32,
    ///
    #[prost(int32, tag = "13")]
    pub arc_pay: i32,
    ///
    #[prost(int32, tag = "14")]
    pub pay_free_watch: i32,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonTheme {
    ///
    #[prost(string, tag = "1")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub selected_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaffInfo {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub attribute: i64,
}
///
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Stat {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub view: i32,
    ///
    #[prost(int32, tag = "3")]
    pub danmaku: i32,
    ///
    #[prost(int32, tag = "4")]
    pub reply: i32,
    ///
    #[prost(int32, tag = "5")]
    pub fav: i32,
    ///
    #[prost(int32, tag = "6")]
    pub coin: i32,
    ///
    #[prost(int32, tag = "7")]
    pub share: i32,
    ///
    #[prost(int32, tag = "8")]
    pub now_rank: i32,
    ///
    #[prost(int32, tag = "9")]
    pub his_rank: i32,
    ///
    #[prost(int32, tag = "10")]
    pub like: i32,
    ///
    #[prost(int32, tag = "11")]
    pub dislike: i32,
}
