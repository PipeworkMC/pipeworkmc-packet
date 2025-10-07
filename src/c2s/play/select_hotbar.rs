//! Serverbound play select hotbar packet.


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
use pipeworkmc_data::selected_hotbar::{
    SelectedHotbar,
    SelectedHotbarDecodeError
};


/// The player changed their selected hotbar slot.
#[derive(Debug)]
pub struct S2CPlaySelectHotbarPacket {
    /// The selected slot.
    pub slot : SelectedHotbar
}

impl PacketMeta for S2CPlaySelectHotbarPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("set_carried_item");
}

impl PacketDecode for S2CPlaySelectHotbarPacket {
    type Error = SelectedHotbarDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        slot : <_>::decode(iter)?
    }) }
}
