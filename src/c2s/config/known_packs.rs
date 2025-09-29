//! Serverbound configuration known packs packet.


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
use pipeworkmc_data::known_pack::KnownPack;


/// Tells the server what data packs are present in the game.
#[derive(Debug)]
pub struct C2SConfigKnownPacksPacket {
    /// The known data packs.
    pub known_packs : Vec<KnownPack<'static>>
}

impl PacketMeta for C2SConfigKnownPacksPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x07; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigKnownPacksPacket {
    type Error = <Vec<KnownPack<'static>> as PacketDecode>::Error;

    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        known_packs : <_>::decode(iter)?,
    }) }
}
