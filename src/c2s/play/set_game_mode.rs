//! Serverbound play set game mode packet.


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
use pipeworkmc_data::game_mode::{
    GameMode,
    GameModeDecodeError
};


/// Tells the server to change the player's game mode.
#[derive(Debug)]
pub struct C2SPlaySetGameModePacket {
    /// The game mode to switch to.
    pub game_mode : GameMode
}

impl PacketMeta for C2SPlaySetGameModePacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("change_game_mode");
}

impl PacketDecode for C2SPlaySetGameModePacket {
    type Error = GameModeDecodeError;

    #[inline]
    fn decode<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    { Ok(Self {
        game_mode : <_>::decode(iter)?
    }) }
}
