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


#[derive(Debug)]
pub enum S2CPlayGameEventPacket {
    NoRespawnBlock,
    BeginRain,
    EndRain,
    ChangeGameMode {
        to : GameMode
    },
    WinGame {
        show_credits : bool
    },
    Demo(DemoEvent),
    ArrowHitPlayer,
    RainLevel {
        level : f32
    },
    ThunderLevel {
        level : f32
    },
    PufferfishStingSound,
    ElderGuardianEffect,
    SetRespawnScreen {
        show_respawn_screen : bool
    },
    SetLimitedCrafting {
        is_crafting_limited : bool
    },
    WaitForChunks
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum DemoEvent {
    Welcome          = 0,
    MovementControls = 101,
    JumpControl      = 102,
    InventoryControl = 103,
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
    const PREFIX : u8          = 0x22; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayGameEventPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        let (b, v,) = self.raw();
        b.encode_len() + v.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        let (b, v,) = self.raw();
        b.encode(buf);
        v.encode(buf);
    } }

}

impl From<S2CPlayGameEventPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayGameEventPacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayGameEventPacket> for S2CPlayPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayGameEventPacket) -> Self { Self::GameEvent(value) }
}
