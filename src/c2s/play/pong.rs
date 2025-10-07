//! Serverbound play pong packet.


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
pub struct C2SPlayPongPacket {
    /// Transaction ID of the ping. The server previously sent the value to use.
    pub transaction : u32
}

impl PacketMeta for C2SPlayPongPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("pong");
}

impl PacketDecode for C2SPlayPongPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        transaction : <_>::decode(iter)?
    }) }
}
