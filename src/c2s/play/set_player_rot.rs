//! Serverbound play set player rot packet.


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


/// The player changed rotation.
#[derive(Debug)]
pub struct C2SPlaySetPlayerRotPacket {
    /// Rotation yaw to rotate to.
    pub yaw   : f32,
    /// Rotation pitch to rotate to.
    pub pitch : f32,
    /// Movement flags.
    pub flags : CharacterMoveFlags
}

impl PacketMeta for C2SPlaySetPlayerRotPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("move_player_rot");
}

impl PacketDecode for C2SPlaySetPlayerRotPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        yaw   : <_>::decode(iter)?,
        pitch : <_>::decode(iter)?,
        flags : <_>::decode(iter)?
    }) }
}
