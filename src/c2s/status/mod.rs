//! Serverbound status packets.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::PacketMeta
};
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


pub mod request;
pub mod ping;


/// Serverbound status packets.
#[derive(Debug)]
pub enum C2SStatusPackets {
    /// Request
    Request(request::C2SStatusRequestPacket),
    /// Ping
    Ping(ping::C2SStatusPingPacket)
}

impl PrefixedPacketDecode for C2SStatusPackets {
    type Error = C2SStatusDecodeError;

    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (iter.read().map_err(C2SStatusDecodeError::Prefix)?) {
            request ::C2SStatusRequestPacket ::PREFIX => Self::Request (request ::C2SStatusRequestPacket ::decode(iter)?),
            ping    ::C2SStatusPingPacket    ::PREFIX => Self::Ping    (ping    ::C2SStatusPingPacket    ::decode(iter).map_err(C2SStatusDecodeError::Ping)?),

            v => { return Err(C2SStatusDecodeError::UnknownPrefix(v)); }
        })
    }
}


/// Returned by packet decoders when a `C2SStatusPackets` was not decoded successfully.
#[derive(Debug)]
pub enum C2SStatusDecodeError {
    /// There were not enough bytes to decode a packet ID.
    Prefix(IncompleteDecodeError),
    /// A `C2SStatusPingPacket` failed to decode.
    Ping(IncompleteDecodeError),
    /// An unrecognised packet ID was found.
    UnknownPrefix(u8)
}
impl From<!> for C2SStatusDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SStatusDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Prefix(err)       => err.fmt(f),
        Self::Ping (err)        => write!(f, "ping {err}"),
        Self::UnknownPrefix (b) => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
