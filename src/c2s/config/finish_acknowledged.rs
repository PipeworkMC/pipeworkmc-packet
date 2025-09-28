//! Serverbound configuration finish acknowledged packet.


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


/// The client has recognised that configuration has been completed.
#[derive(Debug)]
pub struct C2SConfigFinishAcknowledgedPacket;

impl PacketMeta for C2SConfigFinishAcknowledgedPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x03; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigFinishAcknowledgedPacket {
    type Error = !;

    #[inline(always)]
    fn decode<I>(_ : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self) }
}
