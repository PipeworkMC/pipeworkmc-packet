//! Serverbound status request packet.


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


/// Requests server list information from the server.
#[derive(Debug)]
pub struct C2SStatusRequestPacket;

impl PacketMeta for C2SStatusRequestPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for C2SStatusRequestPacket {
    type Error = !;

    #[inline]
    fn decode<I>(_ : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self) }
}
