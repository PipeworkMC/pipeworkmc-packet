//! Serverbound configuration packets.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeIter,
        IncompleteDecodeError,
        vec::VecDecodeError
    },
    meta::PacketMeta
};
use pipeworkmc_data::{
    client_info::ClientInfoDecodeError,
    channel_data::ChannelDataDecodeError,
    known_pack::KnownPackDecodeError
};
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


pub mod client_info;
pub mod custom_payload;
pub mod finish_acknowledged;
pub mod keep_alive;
pub mod known_packs;


/// Serverbound configuration packets.
#[derive(Debug)]
pub enum C2SConfigPackets {
    /// Client info
    ClientInfo(client_info::C2SConfigClientInfoPacket),
    // TODO: CookieResponse
    /// Custom payload
    CustomPayload(custom_payload::C2SConfigCustomPayloadPacket),
    /// Finish acknowledged
    FinishAcknowledged(finish_acknowledged::C2SConfigFinishAcknowledgedPacket),
    /// Keep alive
    KeepAlive(keep_alive::C2SConfigKeepAlivePacket),
    // TODO: Pong
    // TODO: ResourcePack
    /// Known packs
    KnownPacks(known_packs::C2SConfigKnownPacksPacket)
    // TODO: CustomClickAction
}

impl PrefixedPacketDecode for C2SConfigPackets {
    type Error = C2SConfigDecodeError;

    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (iter.read().map_err(C2SConfigDecodeError::Prefix)?) {
            client_info         ::C2SConfigClientInfoPacket         ::PREFIX => Self::ClientInfo         (client_info         ::C2SConfigClientInfoPacket         ::decode(iter).map_err(C2SConfigDecodeError::ClientInfo)?),
            custom_payload      ::C2SConfigCustomPayloadPacket      ::PREFIX => Self::CustomPayload      (custom_payload      ::C2SConfigCustomPayloadPacket      ::decode(iter).map_err(C2SConfigDecodeError::CustomPayload)?),
            finish_acknowledged ::C2SConfigFinishAcknowledgedPacket ::PREFIX => Self::FinishAcknowledged (finish_acknowledged ::C2SConfigFinishAcknowledgedPacket ::decode(iter)?),
            keep_alive          ::C2SConfigKeepAlivePacket          ::PREFIX => Self::KeepAlive          (keep_alive          ::C2SConfigKeepAlivePacket          ::decode(iter).map_err(C2SConfigDecodeError::KeepAlive)?),
            known_packs         ::C2SConfigKnownPacksPacket         ::PREFIX => Self::KnownPacks         (known_packs         ::C2SConfigKnownPacksPacket         ::decode(iter).map_err(C2SConfigDecodeError::KnownPacks)?),

            v => { return Err(C2SConfigDecodeError::UnknownPrefix(v)); }
        })
    }
}


/// Returned by packet decoders when a `C2SConfigPackets` was not decoded successfully.
#[derive(Debug)]
pub enum C2SConfigDecodeError {
    /// There were not enough bytes to decode a packet ID.
    Prefix(IncompleteDecodeError),
    /// A `C2SConfigClientInfoPacket` failed to decode.
    ClientInfo(ClientInfoDecodeError),
    /// A `C2SConfigCustomPayloadPacket` failed to decode.
    CustomPayload(ChannelDataDecodeError),
    /// A `C2SConfigKeepAlivePacket` failed to decode.
    KeepAlive(IncompleteDecodeError),
    /// A `C2SConfigKnownPacksPacket` failed to decode.
    KnownPacks(VecDecodeError<KnownPackDecodeError>),
    /// An unrecognised packet ID was found.
    UnknownPrefix(u8)
}
impl From<!> for C2SConfigDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SConfigDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Prefix(err)        => err.fmt(f),
        Self::ClientInfo(err)    => write!(f, "client info {err}"),
        Self::CustomPayload(err) => write!(f, "custom payload {err}"),
        Self::KeepAlive(err)     => write!(f, "keep alive {err}"),
        Self::KnownPacks(err)    => write!(f, "known packs {err}"),
        Self::UnknownPrefix(b)   => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
