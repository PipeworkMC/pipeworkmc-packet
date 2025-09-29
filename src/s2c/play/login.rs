//! Clientbound play login packet.


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
    block_pos::DimBlockPos,
    character::CharacterId,
    ident::Ident,
    game_mode::GameMode,
    varint::VarInt
};
use std::borrow::Cow;


/// Finalises the player login process.
#[derive(Debug)]
pub struct S2CPlayLoginPacket<'l> {
    /// The player's character ID.
    pub eid                   : CharacterId,
    /// Whether the server is in hardcore.
    pub hardcore              : bool,
    /// [`Ident`]s of all dimension on the server.
    pub all_dim_ids           : Cow<'l, [Ident]>,
    /// Seemingly unused by the game.
    pub max_players           : u32,
    /// Maximum render distance allowed by the server.
    pub view_dist             : u32,
    /// The distance that the client should process things like characters.
    pub sim_dist              : u32,
    /// Decreases the amount of information provided in the F3 debug screen.
    pub reduced_debug_info    : bool,
    /// Whether to show the respawn screen on death.
    pub respawn_screen        : bool,
    /// Seemingly unused by the game.
    pub limited_crafting      : bool,
    /// Registry ID of the player's current dimension type.
    pub dim_type              : u32,
    /// [`Ident`] of the player's current dimension.
    pub dim_id                : Ident,
    /// Used by the client for biome noise.
    pub hashed_seed           : u64,
    /// The player's current game mode.
    pub game_mode             : GameMode,
    /// The player's previous game mode.
    ///
    /// Used by the `F3+N` and `F3+F4` game mode switchers.
    pub prev_game_mode        : Option<GameMode>,
    /// Whether the player is in a debug world.
    pub is_debug_world        : bool,
    /// Whether the player is in a superflat world.
    pub is_flat_world         : bool,
    /// The location where the player last died.
    pub death_location        : Option<DimBlockPos>,
    /// Seemingly unused by the game.
    pub portal_cooldown       : u32,
    /// The sea level of the dimension.
    pub sea_level             : i32,
    /// Whether chat signing is required.
    ///
    /// Chat signing is not supported by pipework.
    pub requires_chat_signing : bool
}

impl PacketMeta for S2CPlayLoginPacket<'_> {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x2B; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayLoginPacket<'_> {

    fn encode_len(&self) -> usize {
        self.eid.as_u32().encode_len()
        + self.hardcore.encode_len()
        + self.all_dim_ids.encode_len()
        + VarInt::<u32>(self.max_players).encode_len()
        + VarInt::<u32>(self.view_dist).encode_len()
        + VarInt::<u32>(self.sim_dist).encode_len()
        + self.reduced_debug_info.encode_len()
        + self.respawn_screen.encode_len()
        + self.limited_crafting.encode_len()
        + VarInt::<u32>(self.dim_type).encode_len()
        + self.dim_id.encode_len()
        + self.hashed_seed.encode_len()
        + (self.game_mode as u8).encode_len()
        + self.prev_game_mode.map_or(-1, |g| g as i8).encode_len()
        + self.is_debug_world.encode_len()
        + self.is_flat_world.encode_len()
        + self.death_location.encode_len()
        + VarInt::<u32>(self.portal_cooldown).encode_len()
        + VarInt::<i32>(self.sea_level).encode_len()
        + self.requires_chat_signing.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.eid.as_u32().encode(buf);
        self.hardcore.encode(buf);
        self.all_dim_ids.encode(buf);
        VarInt::<u32>(self.max_players).encode(buf);
        VarInt::<u32>(self.view_dist).encode(buf);
        VarInt::<u32>(self.sim_dist).encode(buf);
        self.reduced_debug_info.encode(buf);
        self.respawn_screen.encode(buf);
        self.limited_crafting.encode(buf);
        VarInt::<u32>(self.dim_type).encode(buf);
        self.dim_id.encode(buf);
        self.hashed_seed.encode(buf);
        (self.game_mode as u8).encode(buf);
        self.prev_game_mode.map_or(-1, |g| g as i8).encode(buf);
        self.is_debug_world.encode(buf);
        self.is_flat_world.encode(buf);
        self.death_location.encode(buf);
        VarInt::<u32>(self.portal_cooldown).encode(buf);
        VarInt::<i32>(self.sea_level).encode(buf);
        self.requires_chat_signing.encode(buf);
    } }

}

impl<'l> From<S2CPlayLoginPacket<'l>> for S2CPackets<'l> {
    #[inline]
    fn from(value : S2CPlayLoginPacket<'l>) -> Self { Self::Play(value.into()) }
}

impl<'l> From<S2CPlayLoginPacket<'l>> for S2CPlayPackets<'l> {
    #[inline]
    fn from(value : S2CPlayLoginPacket<'l>) -> Self { Self::Login(value) }
}
