//! Serverbound play acknowledge chat packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    },
    varint::{
        VarInt,
        VarIntDecodeError
    }
};


/// Lets the server know that some previous chat messages has been received.
#[derive(Debug)]
pub struct C2SPlayAcknowledgeChatPacket {
    /// Number of messages that have been acknowledged.
    pub count : u32
}

impl PacketMeta for C2SPlayAcknowledgeChatPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("chat_ack");
}

impl PacketDecode for C2SPlayAcknowledgeChatPacket {
    type Error = VarIntDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        count : <VarInt<u32>>::decode(iter)?.0
    }) }
}
