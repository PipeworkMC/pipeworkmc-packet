//! Serverbound play set player pos packet.


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
use pipeworkmc_data::character::{
    CharacterPos,
    CharacterMoveFlags
};


/// The player changed position.
#[derive(Debug)]
pub struct C2SPlaySetPlayerPosPacket {
    /// Position to move to.
    pub pos   : CharacterPos,
    /// Movement flags.
    pub flags : CharacterMoveFlags
}

impl PacketMeta for C2SPlaySetPlayerPosPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("move_player_pos");
}

impl PacketDecode for C2SPlaySetPlayerPosPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        pos   : CharacterPos {
            x : <_>::decode(iter)?,
            y : <_>::decode(iter)?,
            z : <_>::decode(iter)?
        },
        flags : <_>::decode(iter)?
    }) }
}
