//! Serverbound play swing arm packet.


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
use pipeworkmc_data::hand::{
    Hand,
    HandDecodeError
};


/// The player swung one of their arms.
#[derive(Debug)]
pub struct C2SPlaySwingArmPacket {
    /// The arm to swing.
    pub hand : Hand
}

impl PacketMeta for C2SPlaySwingArmPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("swing");
}

impl PacketDecode for C2SPlaySwingArmPacket {
    type Error = HandDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        hand : <_>::decode(iter)?
    }) }
}
