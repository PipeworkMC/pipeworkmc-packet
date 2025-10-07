//! Clientbound play character event packet.


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
use pipeworkmc_data::character::CharacterId;
use core::ops::Deref;


/// Triggers an animation for the character.
#[derive(Debug)]
pub struct S2CPlayCharacterEventPacket {
    /// The ID of the character.
    pub eid    : CharacterId,
    /// What to do.
    pub status : CharacterStatus
}

/// Character statuses.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct CharacterStatus(u8);
impl Deref for CharacterStatus {
    type Target = u8;
    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl CharacterStatus {

    /// Displays honey block slide particles at the entity's feet.
    ///
    /// Applicable to `Entity`.
    pub const HONEY_SLIDE : Self = Self(53);

    /// Displays snowball poof particles.
    ///
    /// Applicable to `SnowballProjectile`.
    pub const SNOWBALL : Self = Self(3);
    /// Spawns item crack particles.
    ///
    /// Applicable to `EggProjectile`.
    pub const EGG : Self = Self(3);

    /// Pulls the caught entity towards the caster.
    ///
    /// Applicable to `FishingHookEntity`.
    pub const FISHING_HOOK : Self = Self(31);

    /// Displays tipped arrow particles.
    ///
    /// Applicable to `ArrowEntity`.
    pub const TIPPED_ARROW : Self = Self(0);

    /// Detonates the firework.
    ///
    /// Applicable to `FireworkRocket`.
    pub const FIREWORK_EXPLODE : Self = Self(17);

    /// Plays the death sound and animation.
    ///
    /// Applicable to `LivingEntity`.
    pub const DEATH : Self = Self(3);
    /// Plays the shield block sound.
    ///
    /// Applicable to `LivingEntity`.
    pub const SHIELD_BLOCK : Self = Self(29);
    /// Plays the shield disabled sound.
    ///
    /// Applicable to `LivingEntity`.
    pub const SHIELD_DISABLED : Self = Self(30);
    /// Plays the totem of undying sound and animation.
    ///
    /// Applicable to `LivingEntity`.
    pub const USE_TOTEM : Self = Self(35);
    /// Displays portal particles when teleporting.
    /// Used by enderman and chorus fruit.
    ///
    /// Applicable to `LivingEntity`.
    pub const TELEPORT_EFFECT : Self = Self(46);
    /// Plays the equipment break sound and displays break particles for the main hand item.
    ///
    /// Applicable to `LivingEntity`.
    pub const BREAK_MAIN_HAND : Self = Self(47);
    /// Plays the equipment break sound and displays break particles for the off hand item.
    ///
    /// Applicable to `LivingEntity`.
    pub const BREAK_OFF_HAND : Self = Self(48);
    /// Plays the equipment break sound and displays break particles for the helmet item.
    ///
    /// Applicable to `LivingEntity`.
    pub const BREAK_HELMET : Self = Self(49);
    /// Plays the equipment break sound and displays break particles for the chestplate item.
    ///
    /// Applicable to `LivingEntity`.
    pub const BREAK_CHESTPLATE : Self = Self(50);
    /// Plays the equipment break sound and displays break particles for the leggings item.
    ///
    /// Applicable to `LivingEntity`.
    pub const BREAK_LEGGINGS : Self = Self(51);
    /// Plays the equipment break sound and displays break particles for the boots item.
    ///
    /// Applicable to `LivingEntity`.
    pub const BREAK_BOOTS : Self = Self(52);
    /// Displays honey block fall particles at the entity's feet.
    ///
    /// Applicable to `LivingEntity`.
    pub const HONEY_FALL : Self = Self(54);
    /// Swap the entity's hand items.
    ///
    /// Applicable to `LivingEntity`.
    pub const SWAP_HANDS : Self = Self(55);
    /// Displays death smoke particles.
    ///
    /// Applicable to `LivingEntity`.
    pub const DEATH_SMOKE : Self = Self(60);

    /// Marks item use (eating, drinking, etc) as finished.
    ///
    /// Applicable to `PlayerEntity`.
    pub const ITEM_USE_FINISH : Self = Self(9);
    /// Decreases the amount of information available on the `F3` debug screen.
    ///
    /// Applicable to `PlayerEntity`.
    pub const ENABLE_REDUCED_DEBUG_INFO : Self = Self(22);
    /// Increases the amount of information available on the `F3` debug screen.
    ///
    /// Applicable to `PlayerEntity`.
    pub const DISABLE_REDUCED_DEBUG_INFO : Self = Self(23);
    /// Sets the player's operator permission level to 0 (All).
    ///
    /// Applicable to `PlayerEntity`.
    pub const SET_OP_LEVEL_0 : Self = Self(24);
    /// Sets the player's operator permission level to 1 (Moderator).
    ///
    /// Applicable to `PlayerEntity`.
    pub const SET_OP_LEVEL_1 : Self = Self(25);
    /// Sets the player's operator permission level to 2 (Gamemaster).
    ///
    /// Applicable to `PlayerEntity`.
    pub const SET_OP_LEVEL_2 : Self = Self(26);
    /// Sets the player's operator permission level to 3 (Admin).
    ///
    /// Applicable to `PlayerEntity`.
    pub const SET_OP_LEVEL_3 : Self = Self(27);
    /// Sets the player's operator permission level to 4 (Owner).
    ///
    /// Applicable to `PlayerEntity`.
    pub const SET_OP_LEVEL_4 : Self = Self(28);
    /// Spawns cloud particles at the player when the bad omen status effect is triggered.
    ///
    /// Applicable to `PlayerEntity`.
    pub const BAD_OMEN_ACTIVATE : Self = Self(43);

    /// Plays the hit sound.
    ///
    /// Applicable to `ArmourStandEntity`.
    pub const ARMOUR_STAND_HIT : Self = Self(32);

    /// Displays an explosion particle.
    /// Used when a silverfish enter/exits a block, or when spawned by a mob spawner.
    ///
    /// Applicable to `MobEntity`.
    pub const MOB_EXPLODE : Self = Self(20);

    /// Resets the rotation of a squid.
    ///
    /// Applicable to `SquidEntity`.
    pub const RESET_SQUID_ROTATION : Self = Self(19);
    /// Displays "happy villager" particles.
    ///
    /// Applicable to `DolphinEntity`.
    pub const DOLPHIN_LOCATE : Self = Self(38);

    /// Displays heart particles.
    ///
    /// Applicable to `AnimalEntity`.
    pub const ANIMAL_LOVE : Self = Self(18);
    /// Displays smoke particles.
    ///
    /// Applicable to `AbstractHorseEntity`.
    pub const HORSE_TAME_FAIL : Self = Self(6);
    /// Displays heart particles.
    ///
    /// Applicable to `AbstractHorseEntity`.
    pub const HORSE_TAME_SUCCESS : Self = Self(7);
    /// Displays smoke particles.
    ///
    /// Applicable to `OcelotEntity`.
    pub const OCELOT_TAME_FAIL : Self = Self(40);
    /// Displays heart particles.
    ///
    /// Applicable to `OcelotEntity`.
    pub const OCELOT_TAME_SUCCESS : Self = Self(41);
    /// Plays the jumping animation and displays jumping particles.
    ///
    /// Applicable to `OcelotEntity`.
    pub const RABBIT_JUMP : Self = Self(1);
    /// Plays the sheep eating grass animation.
    /// The animation lasts 40 ticks.
    ///
    /// Applicable to `SheepEntity`.
    pub const SHEEP_EAT : Self = Self(10);
    /// Spawns item particles based on the item in the fox's mouth (main hand).
    ///
    /// Applicable to `FoxEntity`.
    pub const FOX_CHEW : Self = Self(45);
    /// Begins lowering the goat's head to indicate ramming.
    ///
    /// Applicable to `GoatEntity`.
    pub const GOAT_BEGIN_RAM : Self = Self(58);
    /// Cancel lowering the goat's head.
    ///
    /// Applicable to `GoatEntity`.
    pub const GOAT_CANCEL_RAM : Self = Self(59);
    /// Displays smoke particles.
    ///
    /// Applicable to `TameableAnimalEntity`.
    pub const TAMEABLE_TAME_FAIL : Self = Self(6);
    /// Displays heart particles.
    ///
    /// Applicable to `TameableAnimalEntity`.
    pub const TAMEABLE_TAME_SUCCESS : Self = Self(7);
    /// Plays the wolf shaking water animation.
    ///
    /// Applicable to `WolfEntity`.
    pub const WOLF_BEGIN_SHAKE : Self = Self(8);
    /// Stops the wolf shaking water animation.
    ///
    /// Applicable to `WolfEntity`.
    pub const WOLF_STOP_SHAKE : Self = Self(56);

    /// Displays heart particles.
    ///
    /// Applicable to `VillagerEntity`.
    pub const VILLAGER_HEARTS : Self = Self(12);
    /// Displays villager angry particles.
    ///
    /// Applicable to `VillagerEntity`.
    pub const VILLAGER_ANGRY : Self = Self(13);
    /// Displays villager happy particles.
    ///
    /// Applicable to `VillagerEntity`.
    pub const VILLAGER_HAPPY : Self = Self(14);
    /// Displays splash particles.
    ///
    /// Applicable to `VillagerEntity`.
    pub const VILLAGER_SWEAT : Self = Self(42);

    /// Plays the attack sound and animation.
    ///
    /// Applicable to `IronGolemEntity`.
    pub const IRON_GOLEM_ATTACK : Self = Self(4);
    /// Causes the golem to hold out a poppy.
    /// Animation lasts 400 ticks.
    ///
    /// Applicable to `IronGolemEntity`.
    pub const IRON_GOLEM_OFFER_FLOWER : Self = Self(11);
    /// Causes the golem to put the held poppy away.
    ///
    /// Applicable to `IronGolemEntity`.
    pub const IRON_GOLEM_CANCEL_FLOWER : Self = Self(34);

    /// Plays the attack sound and animation.
    ///
    /// Applicable to `EvokerFangsEntity`.
    pub const EVOKER_FANGS_ATTACK : Self = Self(4);

    /// Displays magic particles.
    ///
    /// Applicable to `WitchEntity`.
    pub const WITCH_MAGIC : Self = Self(15);
    /// Plays the attack animation.
    ///
    /// Applicable to `RavagerEntity`.
    pub const RAVAGER_ATTACK : Self = Self(4);
    /// Plays the stunned animation.
    /// Animation lasts 40 ticks.
    ///
    /// Applicable to `RavagerEntity`.
    pub const RAVAGER_STUN : Self = Self(39);
    /// Plays the cure start sound.
    ///
    /// Applicable to `ZombieVillagerEntity`.
    pub const ZOMBINE_VILLAGER_CURE : Self = Self(16);
    /// Plays the attack sound.
    ///
    /// Applicable to `GuardianEntity`.
    pub const GUARDIAN_ATTACK : Self = Self(21);

    /// Ignites a TNT minecart.
    /// Does not play the sound.
    ///
    /// Applicable to `TntMinecartEntity`
    pub const MINECART_TNT_IGNITE : Self = Self(10);
    /// Resets the spawner minecart spawn timer.
    ///
    /// Applicable to `SpawnerMinecartEntity`
    pub const MINECART_SPAWNER_RESET : Self = Self(1);

    /// Plays the attack sound and animation.
    ///
    /// Applicable to `HoglinEntity`.
    pub const HOGLIN_ATTACK : Self = Self(4);
    /// Plays the attack sound and animation.
    ///
    /// Applicable to `ZoglinEntity`.
    pub const ZOGLIN_ATTACK : Self = Self(4);

    /// Displays heart particles.
    ///
    /// Applicable to `AllayEntity`.
    pub const ALLAY_DUPLICATE : Self = Self(4);

    /// Plays the attack animation.
    ///
    /// Applicable to `WardenEntity`.
    pub const WARDEN_ATTACK : Self = Self(4);
    /// Plays the tendril shake animation.
    /// Animation lasts 10 ticks.
    ///
    /// Applicable to `WardenEntity`.
    pub const WARDEN_VIBRATION : Self = Self(61);
    /// Plays the sonic boom animation.
    /// Beam sound is not included.
    ///
    /// Applicable to `WardenEntity`.
    pub const WARDEN_SONIC_BOOK : Self = Self(62);

    /// Plays the dig animation.
    /// Only works if the sniffer has a target and is either digging or searching.
    ///
    /// Applicable to `SnifferEntity`.
    pub const SNIFFER_DIG : Self = Self(63);
}


impl PacketMeta for S2CPlayCharacterEventPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = super::packet_id!("entity_event");
}

unsafe impl PacketEncode for S2CPlayCharacterEventPacket {

    #[inline]
    fn encode_len(&self) -> usize {
        self.eid.0.encode_len()
    }

    #[inline]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.eid.0.encode(buf);
        (*self.status).encode(buf);
    } }

}

impl From<S2CPlayCharacterEventPacket> for S2CPackets<'_> {
    #[inline]
    fn from(value : S2CPlayCharacterEventPacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayCharacterEventPacket> for S2CPlayPackets<'_> {
    #[inline]
    fn from(value : S2CPlayCharacterEventPacket) -> Self { Self::CharacterEvent(value) }
}
