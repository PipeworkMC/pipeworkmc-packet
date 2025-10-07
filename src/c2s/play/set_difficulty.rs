//! Serverbound play set difficulty packet.


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
use pipeworkmc_data::difficulty::{
    Difficulty,
    DifficultyDecodeError
};


/// Tells the server to change the world difficulty.
#[derive(Debug)]
pub struct C2SPlaySetDifficultyPacket {
    /// The difficulty to switch to.
    pub difficulty : Difficulty
}

impl PacketMeta for C2SPlaySetDifficultyPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("change_difficulty");
}

impl PacketDecode for C2SPlaySetDifficultyPacket {
    type Error = DifficultyDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        difficulty : <_>::decode(iter)?
    }) }
}
