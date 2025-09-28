//! Serverbound login packets.


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


pub mod start;
pub mod encrypt_response;
pub mod finish_acknowledged;


/// Serverbound login packets.
#[derive(Debug)]
pub enum C2SLoginPackets {
    /// Start
    Start(start::C2SLoginStartPacket),
    /// Encrypt response
    EncryptResponse(encrypt_response::C2SLoginEncryptResponsePacket),
    // TODO: QueryResponse
    /// Finish acknowledged.
    FinishAcknowledged(finish_acknowledged::C2SLoginFinishAcknowledgedPacket)
    // TODO: Cookie response
}

impl PrefixedPacketDecode for C2SLoginPackets {
    type Error = C2SLoginDecodeError;

    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (iter.read().map_err(C2SLoginDecodeError::Prefix)?) {
            start               ::C2SLoginStartPacket              ::PREFIX => Self::Start              (start               ::C2SLoginStartPacket              ::decode(iter).map_err(C2SLoginDecodeError::Start)?),
            encrypt_response    ::C2SLoginEncryptResponsePacket    ::PREFIX => Self::EncryptResponse    (encrypt_response    ::C2SLoginEncryptResponsePacket    ::decode(iter).map_err(C2SLoginDecodeError::EncryptResponse)?),
            finish_acknowledged ::C2SLoginFinishAcknowledgedPacket ::PREFIX => Self::FinishAcknowledged (finish_acknowledged ::C2SLoginFinishAcknowledgedPacket ::decode(iter)?),

            v => { return Err(C2SLoginDecodeError::UnknownPrefix(v)); }
        })
    }
}


/// Returned by packet decoders when a `C2SLoginPackets` was not decoded successfully.
#[derive(Debug)]
pub enum C2SLoginDecodeError {
    /// There were not enough bytes to decode a packet ID.
    Prefix(IncompleteDecodeError),
    /// A `C2SLoginStartPacket` failed to decode.
    Start(start::C2SLoginStartDecodeError),
    /// A `C2SLoginEncryptResponsePacket` failed to decode.
    EncryptResponse(encrypt_response::C2SLoginEncryptResponseDecodeError),
    /// An unrecognised packet ID was found.
    UnknownPrefix(u8)
}
impl From<!> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SLoginDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Prefix(err)          => err.fmt(f),
        Self::Start(err)           => write!(f, "start {err}"),
        Self::EncryptResponse(err) => write!(f, "encrypt response {err}"),
        Self::UnknownPrefix (b)    => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
