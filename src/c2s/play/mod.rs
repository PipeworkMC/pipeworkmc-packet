//! Serverbound play packets.


use pipeworkmc_codec::{
    decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeIter,
        IncompleteDecodeError
    },
    meta::PacketMeta
};
use pipeworkmc_data::channel_data::ChannelDataDecodeError;
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


pub mod tick_end;
pub mod custom_payload;
pub mod keep_alive;
pub mod loaded;


/// Serverbound play packets.
#[derive(Debug)]
pub enum C2SPlayPackets {
    // TODO: ConfirmTeleport
    // TODO: QueryBlockEntityTag
    // TODO: BundleItemSelected
    // TODO: ChangeDifficulty
    // TODO: ChangeGameMode
    // TODO: AcknowledgeMessage
    // TODO: ChatCommand
    // TODO: SignedChatCommand
    // TODO: ChatMessage
    // TODO: PlayerSession
    // TODO: ChunkBatchReceived
    // TODO: ClientStatus
    /// Tick end
    TickEnd(tick_end::C2SPlayTickEndPacket),
    // TODO: ClientInfo
    // TODO: CommandSuggestionsRequest
    // TODO: AcknowledgeConfig
    // TODO: ClickContainerButton
    // TODO: ClickContainer
    // TODO: CloseContinaer
    // TODO: ChangeContainerSlotState
    // TODO: CookieResponse
    /// Custom payload
    CustomPayload(custom_payload::C2SPlayCustomPayloadPacket),
    // TODO: DebugSampleSubscription
    // TODO: EditBook
    // TODO: QueryEntityTag
    // TODO: Interact
    // TODO: JigsawGenerate
    /// Keep alive
    KeepAlive(keep_alive::C2SPlayKeepAlivePacket),
    // TODO: LockDifficulty
    // TODO: SetPlayerPos
    // TODO: SetPlayerPosAndRot
    // TODO: SetPlayerRot
    // TODO: SetPlayerMovementFlags
    // TODO: MoveVehicle
    // TODO: PaddleBoat
    // TODO: PickBlock
    // TODO: PickEntity
    // TODO: PingRequest
    // TODO: PlaceRecipe
    // TODO: PlayerAbilities
    // TODO: PlayerAction
    // TODO: PlayerCommand
    // TODO: PlayerInput
    /// Loaded
    Loaded(loaded::C2SPlayLoadedPacket)
    // TODO: Pong
    // TODO: ChangeRecipeBookSettings
    // TODO: SetSeenRecipe
    // TODO: RenameItem
    // TODO: ResourcePackResponse
    // TODO: SeenAdvancements
    // TODO: SelectTrade
    // TODO: SetBeaconEffect
    // TODO: SetHeldItem
    // TODO: ProgramCommandBlock
    // TODO: ProgramCommandBlockMinecart
    // TODO: SetCreativeModeSlot
    // TODO: ProgramJigsawBlock
    // TODO: ProgramStructureBlock
    // TODO: SetTestBlock
    // TODO: UpdateSign
    // TODO: SwingArm
    // TODO: TeleportToEntity
    // TODO: TestInstanceBlockAction
    // TODO: UseItemOn
    // TODO: UseItem
    // TODO: CustomClickAction
}

impl PrefixedPacketDecode for C2SPlayPackets {
    type Error = C2SPlayDecodeError;

    fn decode_prefixed<I>(iter : &mut DecodeIter<I>) -> Result<Self, Self::Error>
    where
        I : ExactSizeIterator<Item = u8>
    {
        Ok(match (iter.read().map_err(C2SPlayDecodeError::Prefix)?) {
            tick_end       ::C2SPlayTickEndPacket       ::PREFIX => Self::TickEnd       (tick_end       ::C2SPlayTickEndPacket       ::decode(iter)?),
            custom_payload ::C2SPlayCustomPayloadPacket ::PREFIX => Self::CustomPayload (custom_payload ::C2SPlayCustomPayloadPacket ::decode(iter).map_err(C2SPlayDecodeError::CustomPayload)?),
            keep_alive     ::C2SPlayKeepAlivePacket     ::PREFIX => Self::KeepAlive     (keep_alive     ::C2SPlayKeepAlivePacket     ::decode(iter).map_err(C2SPlayDecodeError::KeepAlive)?),
            loaded         ::C2SPlayLoadedPacket        ::PREFIX => Self::Loaded        (loaded         ::C2SPlayLoadedPacket        ::decode(iter)?),

            v => { return Err(C2SPlayDecodeError::UnknownPrefix(v)); }
        })
    }
}


/// Returned by packet decoders when a `C2SPlayPackets` was not decoded successfully.
#[derive(Debug)]
pub enum C2SPlayDecodeError {
    /// There were not enough bytes to decode a packet ID.
    Prefix(IncompleteDecodeError),
    /// A `C2SPlayCustomPayloadPacket` failed to decode.
    CustomPayload(ChannelDataDecodeError),
    /// A `C2SPlayKeepAlivePacket` failed to decode.
    KeepAlive(IncompleteDecodeError),
    /// An unrecognised packet ID was found.
    UnknownPrefix(u8)
}
impl From<!> for C2SPlayDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SPlayDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Prefix(err)        => err.fmt(f),
        Self::CustomPayload(err) => write!(f, "custom payload {err}"),
        Self::KeepAlive(err)     => write!(f, "keep alive {err}"),
        Self::UnknownPrefix(b)   => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
