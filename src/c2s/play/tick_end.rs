//! Serverbound play tick end packet.


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


/// The client world has ticked.
#[derive(Debug)]
pub struct C2SPlayTickEndPacket;

impl PacketMeta for C2SPlayTickEndPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("client_tick_end");
}

impl PacketDecode for C2SPlayTickEndPacket {
    type Error = !;

    #[inline]
    fn decode<I>(_ : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self) }
}
