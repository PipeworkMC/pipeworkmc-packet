//! Clientbound play game event packet.


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
use pipeworkmc_data::game_mode::GameMode;


/// Alerts the client that some global event has occured.
#[derive(Debug)]
pub enum S2CPlayGameEventPacket {
    /// Displays the `block.minercaft.spawn.not_valid` message.
    NoRespawnBlock,
    /// Enables rain.
    BeginRain,
    /// Disables rain.
    EndRain,
    /// Sets the player's game mode.
    ChangeGameMode {
        /// The game mode to switch to.
        to : GameMode
    },
    /// Respawns the player.
    WinGame {
        /// Whether to roll the game credits.
        show_credits : bool
    },
    /// Shows demo information.
    Demo(DemoEvent),
    /// Any player was struck by an arrow
    ArrowHitPlayer,
    /// Changes the sky colour and lighting.
    RainLevel {
        /// How heavy it is raining.
        level : f32
    },
    /// Changes the sky colour and lighting.
    ThunderLevel {
        /// How heavy it is thundering.
        level : f32
    },
    /// Plays a pufferfish sting sound.
    PufferfishStingSound,
    /// Plays the elder guardian effect and sound.
    ElderGuardianEffect,
    /// Sets whether the respawns screen is enabled.
    SetRespawnScreen {
        /// Whether the respawn screen should be shown on death.
        show_respawn_screen : bool
    },
    /// Seemingly unused by the game.
    SetLimitedCrafting {
        /// Seemingly unused by the game.
        is_crafting_limited : bool
    },
    /// Instructs the client to wait for world chunks.
    WaitForChunks
}

/// Demo events.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum DemoEvent {
    /// Displays the demo welcome screen.
    Welcome          = 0,
    /// Shows movement controls.
    MovementControls = 101,
    /// Shows the jump control.
    JumpControl      = 102,
    /// Shows the inventory control.
    InventoryControl = 103,
    /// Displays the demo over screen and shows the screenshot control.
    DemoOver         = 104
}


impl S2CPlayGameEventPacket {
    #[inline]
    fn raw(&self) -> (u8, f32,) { match (self) {
        Self::NoRespawnBlock                             => (0, 0.0,),
        Self::BeginRain                                  => (1, 0.0,),
        Self::EndRain                                    => (2, 0.0,),
        Self::ChangeGameMode { to }                      => (3, *to as u8 as f32,),
        Self::WinGame { show_credits }                   => (4, *show_credits as u8 as f32,),
        Self::Demo(demo_event)                           => (5, *demo_event as u8 as f32,),
        Self::ArrowHitPlayer                             => (6, 0.0,),
        Self::RainLevel { level }                        => (7, *level,),
        Self::ThunderLevel { level }                     => (8, *level,),
        Self::PufferfishStingSound                       => (9, 0.0,),
        Self::ElderGuardianEffect                        => (10, 0.0,),
        Self::SetRespawnScreen { show_respawn_screen }   => (11, *show_respawn_screen as u8 as f32,),
        Self::SetLimitedCrafting { is_crafting_limited } => (12, *is_crafting_limited as u8 as f32,),
        Self::WaitForChunks                              => (13, 0.0,),
    } }
}


impl PacketMeta for S2CPlayGameEventPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("game_event");
}

unsafe impl PacketEncode for S2CPlayGameEventPacket {

    #[inline]
    fn encode_len(&self) -> usize {
        let (b, v,) = self.raw();
        b.encode_len() + v.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        let (b, v,) = self.raw();
        b.encode(buf);
        v.encode(buf);
    } }

}

impl From<S2CPlayGameEventPacket> for S2CPackets<'_> {
    #[inline]
    fn from(value : S2CPlayGameEventPacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayGameEventPacket> for S2CPlayPackets<'_> {
    #[inline]
    fn from(value : S2CPlayGameEventPacket) -> Self { Self::GameEvent(value) }
}
