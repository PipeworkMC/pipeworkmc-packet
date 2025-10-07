//! Serverbound play chat command packet.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        DecodeIter,
        string::StringDecodeError
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


/// Tells the server to run a command.
#[derive(Debug)]
pub struct C2SPlayChatCommandPacket {
    /// The command to run without the `/`.
    pub command : String
}

impl PacketMeta for C2SPlayChatCommandPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("chat_command");
}

impl PacketDecode for C2SPlayChatCommandPacket {
    type Error = StringDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        command : <_>::decode(iter)?
    }) }
}
