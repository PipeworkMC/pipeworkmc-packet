//! Clientbound status response packet.


use crate::s2c::{
    S2CPackets,
    status::S2CStatusPackets
};
use pipeworkmc_codec::{
    encode::{
        PacketEncode,
        EncodeBuf
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::{
    bounded_string::BoundedString,
    text::{
        Text,
        TextComponent,
        TextContent
    },
    uuid::Uuid,
    version::Version
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};
use serde_json::to_string as to_json_string;


/// Sends server list information to the client.
#[derive(Debug)]
pub struct S2CStatusResponsePacket<'l> {
    status_json : Cow<'l, str>
}

impl PacketMeta for S2CStatusResponsePacket<'_> {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CStatusResponsePacket<'_> {

    #[inline]
    fn encode_len(&self) -> usize {
        self.status_json.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.status_json.encode(buf);
    } }

}

impl<'l> From<S2CStatusResponsePacket<'l>> for S2CPackets<'l> {
    #[inline]
    fn from(value : S2CStatusResponsePacket<'l>) -> Self { Self::Status(value.into()) }
}

impl<'l> From<S2CStatusResponsePacket<'l>> for S2CStatusPackets<'l> {
    #[inline]
    fn from(value : S2CStatusResponsePacket<'l>) -> Self { Self::Response(value) }
}


/// Server list information.
#[derive(Ser)]
pub struct Status<'l> {
    /// Server version.
    pub version               : StatusVersion<'l>,
    /// Online player information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players               : Option<StatusPlayers>,
    /// The server list message of the day text.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub motd                  : Option<Text>,
    /// A base 64 PNG image.
    ///
    /// Do not include `data:image/png;base64,` at the start.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "add_png_b64_header")]
    pub favicon               : Option<Cow<'l, str>>,
    /// Whether the server requires chat message signatures.
    #[serde(rename = "enforcesSecureChat")]
    pub requires_chat_signing : bool,
    /// Whether the server has a mod like No Chat Reports which strips chat message signatures.
    #[serde(rename = "preventsChatReports")]
    pub prevents_chat_reports : bool
}
fn add_png_b64_header<'l, S : Serer>(png_b64 : &Option<Cow<'l, str>>, ser : S) -> Result<S::Ok, S::Error> {
    if let Some(png_b64) = png_b64 {
        ser.serialize_str(&format!("data:image/png;base64,{png_b64}"))
    } else {
        ser.serialize_none()
    }
}

/// Server list version information.
#[derive(Ser)]
pub struct StatusVersion<'l> {
    /// Name of the version.
    pub name     : Cow<'l, str>,
    /// Protocol ID of the version.
    pub protocol : u32
}

/// Server list player information.
#[derive(Ser)]
pub struct StatusPlayers {
    /// Number of players currently online.
    #[serde(rename = "online")]
    pub current : u32,
    /// Maximum number of players allowed on the server.
    pub max     : u32,
    /// A sample of online players.
    pub sample  : Cow<'static, [StatusPlayer]>
}

/// An entry in the server list player sample.
#[derive(Ser, Clone)]
pub struct StatusPlayer {
    /// UUID of the player.
    #[serde(rename = "id")]
    pub uuid : Uuid,
    /// Name of the player.
    pub name : String
}

/// The name of a player in the server list player sample.
pub enum StatusPlayerName {
    /// String.
    String(String),
    /// BoundedString.
    BoundedString(BoundedString<16>)
}


impl Default for Status<'_> {
    fn default() -> Self { Self {
        version               : StatusVersion::default(),
        players               : None,
        motd                  : Some(Text { components : Cow::Borrowed(&[TextComponent {
            content : TextContent::Literal { text : Cow::Borrowed("A PipeworkMC Server") },
            .. TextComponent::EMPTY
        }]) }),
        favicon               : None,
        requires_chat_signing : false,
        prevents_chat_reports : true
    } }
}

impl Default for StatusVersion<'_> {
    #[inline]
    fn default() -> Self { Self {
        name     : Cow::Borrowed(Version::CURRENT.earliest_name()),
        protocol : Version::CURRENT.id()
    } }
}


impl From<&Status<'_>> for S2CStatusResponsePacket<'_> {
    #[inline]
    fn from(value : &Status) -> Self {
        Self { status_json : Cow::Owned(to_json_string(&value).unwrap()) }
    }
}
impl From<Status<'_>> for S2CStatusResponsePacket<'_> {
    #[inline]
    fn from(value : Status) -> Self {
        Self { status_json : Cow::Owned(to_json_string(&value).unwrap()) }
    }
}

impl From<&Status<'_>> for S2CPackets<'_> {
    #[inline]
    fn from(value : &Status<'_>) -> Self { Self::Status(value.into()) }
}
impl From<Status<'_>> for S2CPackets<'_> {
    #[inline]
    fn from(value : Status<'_>) -> Self { Self::Status(value.into()) }
}

impl From<&Status<'_>> for S2CStatusPackets<'_> {
    #[inline]
    fn from(value : &Status<'_>) -> Self { Self::Response(value.into()) }
}
impl From<Status<'_>> for S2CStatusPackets<'_> {
    #[inline]
    fn from(value : Status<'_>) -> Self { Self::Response(value.into()) }
}
