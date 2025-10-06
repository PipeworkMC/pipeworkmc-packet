//! Clientbound configuration known packs packet.


use crate::s2c::{
    S2CPackets,
    config::S2CConfigPackets
};
use pipeworkmc_codec::{
    encode::{
        PacketEncode,
        EncodeBuf
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::known_pack::KnownPack;
use std::borrow::Cow;


/// Tells the client what data packs are present in the game.
#[derive(Debug)]
pub struct S2CConfigKnownPacksPacket<'l> {
    /// The known data packs.
    pub known_packs : Cow<'l, [KnownPack<'l>]>
}


impl PacketMeta for S2CConfigKnownPacksPacket<'_> {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("select_known_packs");
}

unsafe impl PacketEncode for S2CConfigKnownPacksPacket<'_> {

    #[inline]
    fn encode_len(&self) -> usize {
        self.known_packs.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.known_packs.encode(buf);
    } }

}

impl<'l> From<S2CConfigKnownPacksPacket<'l>> for S2CPackets<'l> {
    #[inline]
    fn from(value : S2CConfigKnownPacksPacket<'l>) -> Self { Self::Config(value.into()) }
}

impl<'l> From<S2CConfigKnownPacksPacket<'l>> for S2CConfigPackets<'l> {
    #[inline]
    fn from(value : S2CConfigKnownPacksPacket<'l>) -> Self { Self::KnownPacks(value) }
}
