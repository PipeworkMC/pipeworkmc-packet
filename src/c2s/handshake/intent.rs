//! Serverbound handshaking intent packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError,
        string::StringDecodeError
    },
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
use pipeworkmc_data::varint::{
    VarInt,
    VarIntDecodeError
};
use core::fmt::{ self, Display, Formatter };


/// Informs the server about what the client intends to do with the current connection.
#[derive(Debug)]
pub struct C2SHandshakeIntentPacket {
    /// The client's game protocol version.
    pub protocol       : VarInt<u32>,
    /// What address the client used to connect to the server.
    pub server_address : String,
    /// What port the client used to connect to the server.
    pub server_port    : u16,
    /// What the client intends to do with the current connection.
    pub intent         : Intent
}

impl PacketMeta for C2SHandshakeIntentPacket {
    const STATE  : PacketState = PacketState::Handshake;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("intention");
}

impl PacketDecode for C2SHandshakeIntentPacket {
    type Error = C2SHandshakeIntentDecodeError;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        protocol       : { <_>::decode(iter).map_err(C2SHandshakeIntentDecodeError::Protocol)? },
        server_address : { <_>::decode(iter).map_err(C2SHandshakeIntentDecodeError::Address)? },
        server_port    : { <_>::decode(iter).map_err(C2SHandshakeIntentDecodeError::Port)? },
        intent         : match (*VarInt::<u32>::decode(iter).map_err(C2SHandshakeIntentDecodeError::Intent)?) {
            1 => Intent::Status,
            2 => Intent::Login { is_transfer : false },
            3 => Intent::Login { is_transfer : true },
            v => { return Err(C2SHandshakeIntentDecodeError::UnknownIntent(v)); }
        }
    }) }
}

unsafe impl PacketEncode for C2SHandshakeIntentPacket {

    fn encode_len(&self) -> usize {
        self.protocol.encode_len()
        + self.server_address.encode_len()
        + self.server_port.encode_len()
        + VarInt::<u32>(self.intent.as_u32()).encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.protocol.encode(buf);
        self.server_address.encode(buf);
        self.server_port.encode(buf);
        VarInt::<u32>(self.intent.as_u32()).encode(buf);
    } }

}


/// What the client intends to do with the current connection.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Intent {
    /// Server list status request.
    Status,
    /// Login as a player.
    Login {
        /// Whether the player is transferring from another server.
        is_transfer : bool
    }
}

impl Intent {
    /// Returns this [`Intent`] as an integer.
    pub fn as_u32(self) -> u32 { match (self) {
        Intent::Status                        => 1,
        Intent::Login { is_transfer : false } => 2,
        Intent::Login { is_transfer : true  } => 3
    } }
}


/// Returned by packet decoders when a `C2SHandshakeIntentPacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SHandshakeIntentDecodeError {
    /// The protocol ID failed to decode.
    Protocol(VarIntDecodeError),
    /// The connection address failed to decode.
    Address(StringDecodeError),
    /// The connection port failed to decode.
    Port(IncompleteDecodeError),
    /// The intent failed to decode.
    Intent(VarIntDecodeError),
    /// An unknown intent was found.
    UnknownIntent(u32)
}
impl Display for C2SHandshakeIntentDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Protocol(err)    => write!(f, "protocol version {err}"),
        Self::Address(err)     => write!(f, "connection address {err}"),
        Self::Port(err)        => write!(f, "connection port {err}"),
        Self::Intent(err)      => write!(f, "intent {err}"),
        Self::UnknownIntent(v) => write!(f, "unknown intent {v}")
    } }
}
