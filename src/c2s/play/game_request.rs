//! Serverbound play game request packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
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


/// The client is requesting something from the server.
#[derive(Debug)]
pub enum C2SPlayGameRequestPacket {
    /// The respawn button was clicked on the death screen.
    Respawn,
    /// The statistics screen is being open.
    GetStats
}

impl PacketMeta for C2SPlayGameRequestPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("client_command");
}

impl PacketDecode for C2SPlayGameRequestPacket {
    type Error = C2SPlayGameRequestDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (<VarInt<u32>>::decode(iter).map_err(C2SPlayGameRequestDecodeError::Request)?.0) {
            0 => Self::Respawn,
            1 => Self::GetStats,
            v => { return Err(C2SPlayGameRequestDecodeError::UnknownRequest(v))?; }
        })
    }
}


/// Returned by packet decoders when a `C2SPlayGameRequestPacket` was not decoded successfully.
#[derive(Debug)]
pub enum C2SPlayGameRequestDecodeError {
    /// The request failed to decode.
    Request(VarIntDecodeError),
    /// An unknown request was found.
    UnknownRequest(u32)
}
impl Display for C2SPlayGameRequestDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Request(err)      => write!(f, "request {err}"),
        Self::UnknownRequest(v) => write!(f, "unknown request {v}")
    } }
}
