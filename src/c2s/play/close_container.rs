//! Serverbound play close container packet.


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


/// Lets the server know that the client closed a contianer screen.
#[derive(Debug)]
pub struct C2SPlayCloseContainerPacket {
    /// ID of the container.
    ///
    /// `0` is player inventory.
    pub container : u32
}

impl PacketMeta for C2SPlayCloseContainerPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("container_close");
}

impl PacketDecode for C2SPlayCloseContainerPacket {
    type Error = VarIntDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        container : <VarInt<u32>>::decode(iter)?.0
    }) }
}
