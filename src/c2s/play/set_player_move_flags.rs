//! Serverbound play set player move flags packet.


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
use pipeworkmc_data::character::CharacterMoveFlags;


/// The player movement flags changed.
#[derive(Debug)]
pub struct C2SPlaySetPlayerMoveFlagsPacket {
    /// Movement flags.
    pub flags : CharacterMoveFlags
}

impl PacketMeta for C2SPlaySetPlayerMoveFlagsPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("move_player_status_only");
}

impl PacketDecode for C2SPlaySetPlayerMoveFlagsPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        flags : <_>::decode(iter)?
    }) }
}
