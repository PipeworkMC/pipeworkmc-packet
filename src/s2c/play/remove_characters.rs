//! Clientbound play remove characters packet.


use crate::s2c::{
    S2CPackets,
    play::S2CPlayPackets
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
use pipeworkmc_data::{
    character::CharacterId,
    varint::VarInt
};
use std::borrow::Cow;


/// Destroys an entity on the client.
#[derive(Debug)]
pub struct S2CPlayRemoveCharactersPacket<'l> {
    /// IDs of characters to remove.
    pub eids : Cow<'l, [CharacterId]>
}

impl PacketMeta for S2CPlayRemoveCharactersPacket<'_> {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x46; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayRemoveCharactersPacket<'_> {

    fn encode_len(&self) -> usize {
        let len = VarInt::<u32>(self.eids.len() as u32).encode_len();
        len + self.eids.iter().map(|eid| VarInt::<u32>(eid.as_u32()).encode_len()).sum::<usize>()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.eids.len() as u32).encode(buf);
        for eid in &*self.eids {
            VarInt::<u32>(eid.as_u32()).encode(buf);
        }
    } }

}

impl<'l> From<S2CPlayRemoveCharactersPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CPlayRemoveCharactersPacket<'l>) -> Self { Self::Play(value.into()) }
}

impl<'l> From<S2CPlayRemoveCharactersPacket<'l>> for S2CPlayPackets<'l> {
    #[inline(always)]
    fn from(value : S2CPlayRemoveCharactersPacket<'l>) -> Self { Self::RemoveCharacters(value) }
}
