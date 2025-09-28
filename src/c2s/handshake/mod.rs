//! Serverbound handshaking packets.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::PacketMeta
};
use core::fmt::{ self, Display, Formatter };


pub mod intention;


/// Serverbound handshaking packets.
#[derive(Debug)]
pub enum C2SHandshakePackets {
    /// Intention
    Intention(intention::C2SHandshakeIntentionPacket)
}

impl PrefixedPacketDecode for C2SHandshakePackets {
    type Error = C2SHandshakeDecodeError;

    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (iter.read().map_err(C2SHandshakeDecodeError::Prefix)?) {
            intention::C2SHandshakeIntentionPacket::PREFIX => Self::Intention(intention::C2SHandshakeIntentionPacket::decode(iter).map_err(C2SHandshakeDecodeError::Intention)?),

            v => { return Err(C2SHandshakeDecodeError::UnknownPrefix(v)); }
        })
    }
}


/// Returned by packet decoders when a `C2SHandshakePackets` was not decoded successfully.
#[derive(Debug)]
pub enum C2SHandshakeDecodeError {
    /// There were not enough bytes to decode a packet ID.
    Prefix(IncompleteDecodeError),
    /// A `C2SHandshakeIntentionPacket` failed to decode.
    Intention(intention::C2SHandshakeIntentionDecodeError),
    /// An unrecognised packet ID was found.
    UnknownPrefix(u8)
}
impl Display for C2SHandshakeDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Prefix(err)   => err.fmt(f),
        Self::Intention(err)    => write!(f, "intention {err}"),
        Self::UnknownPrefix (b) => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
