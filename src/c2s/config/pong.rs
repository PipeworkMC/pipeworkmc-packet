//! Serverbound configuration pong packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


/// Response to a previous ping.
#[derive(Debug)]
pub struct C2SConfigPongPacket {
    /// ID of the ping. The server previously sent the value to use.
    pub id : u32
}

impl PacketMeta for C2SConfigPongPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("pong");
}

impl PacketDecode for C2SConfigPongPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        id : <_>::decode(iter)?
    }) }
}
