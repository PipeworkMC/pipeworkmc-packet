//! Serverbound play loaded packet.


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


/// Alerts the server that the client has loaded in properly.
#[derive(Debug)]
pub struct C2SPlayLoadedPacket;

impl PacketMeta for C2SPlayLoadedPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x2B; // TODO: Check against current datagen.
}

impl PacketDecode for C2SPlayLoadedPacket {
    type Error = !;

    #[inline(always)]
    fn decode<I>(_ : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self) }
}
