//! Serverbound play packets.


pub mod acknowledge_teleport;
pub mod query_block_tag;
// TODO: bundle_item_selected
pub mod set_difficulty;
pub mod set_game_mode;
pub mod acknowledge_chat;
pub mod chat_command;
pub mod signed_chat_command;
pub mod chat_message;
// TODO: player_session
pub mod chunk_batch_received;
pub mod game_request;
pub mod tick_end;
pub mod client_info;
// TODO: command_suggestions_request
// TODO: acknowledge_config
// TODO: click_container_button
// TODO: click_container
pub mod close_container;
// TODO: change_container_slot_state
pub mod cookie_response;
pub mod custom_payload;
// TODO: debug_sample_subscription
// TODO: edit_book
pub mod query_character_tag;
// TODO: interact
// TODO: jigsaw_generate
pub mod keep_alive;
// TODO: lock_difficulty
pub mod set_player_pos;
pub mod set_player_pos_rot;
pub mod set_player_rot;
pub mod set_player_move_flags;
// TODO: move_vehicle
// TODO: paddle_boat
// TODO: pick_block
// TODO: pick_entity
// TODO: ping
// TODO: place_recipe
// TODO: player_abilities
// TODO: player_action
// TODO: player_command
// TODO: player_input
pub mod loaded;
pub mod pong;
// TODO: change_recipe_book_settings
// TODO: set_seen_recipe
// TODO: rename_item
// TODO: resource_pack_response
// TODO: seen_advancements
// TODO: select_trade
// TODO: set_beacon_effect
pub mod select_hotbar;
// TODO: program_command_block
// TODO: program_command_block_minecart
// TODO: set_creative_mode_slot
// TODO: program_jigsaw_block
// TODO: program_structure_block
// TODO: progrma_test_block
// TODO: update_sign
pub mod swing_arm;
// TODO: teleport_to_entity
// TODO: test_instance_block_action
// TODO: use_item_on
// TODO: use_item
// TODO: custom_click_action


super::packet_group!(
    "play" C2SPlayPackets,
    C2SPlayDecodeError,
    {
        "acknowledge teleport"  AcknowledgeTeleport => acknowledge_teleport   ::C2SPlayAcknowledgeTeleportPacket,
        "query block tag"       QueryBlockTag       => query_block_tag        ::C2SPlayQueryBlockTagPacket,
        "set difficulty"        SetDifficulty       => set_difficulty         ::C2SPlaySetDifficultyPacket,
        "set game mode"         SetGameMode         => set_game_mode          ::C2SPlaySetGameModePacket,
        "acknowledge chat"      AcknowledgeChat     => acknowledge_chat       ::C2SPlayAcknowledgeChatPacket,
        "chat command"          ChatCommand         => chat_command           ::C2SPlayChatCommandPacket,
        "signed chat command"   SignedChatCommand   => signed_chat_command    ::C2SPlaySignedChatCommandPacket,
        "chat message"          ChatMessage         => chat_message           ::C2SPlayChatMessagePacket,
        "chunk batch received"  ChunkBatchReceived  => chunk_batch_received   ::C2SPlayChunkBatchReceivedPacket,
        "game request"          GameRequest         => game_request           ::C2SPlayGameRequestPacket,
        "tick end"              TickEnd             => tick_end               ::C2SPlayTickEndPacket,
        "client info"           ClientInfo          => client_info            ::C2SPlayClientInfoPacket,
        "close container"       CloseContainer      => close_container        ::C2SPlayCloseContainerPacket,
        "cookie response"       CookieResponse      => cookie_response        ::C2SPlayCookieResponsePacket,
        "custom payload"        CustomPayload       => custom_payload         ::C2SPlayCustomPayloadPacket,
        "query character tag"   QueryCharacterTag   => query_character_tag    ::C2SPlayQueryCharacterTagPacket,
        "keep alive"            KeepAlive           => keep_alive             ::C2SPlayKeepAlivePacket,
        "set player pos"        SetPlayerPos        => set_player_pos         ::C2SPlaySetPlayerPosPacket,
        "set player pos rot"    SetPlayerPosRot     => set_player_pos_rot     ::C2SPlaySetPlayerPosRotPacket,
        "set player rot"        SetPlayerRot        => set_player_rot         ::C2SPlaySetPlayerRotPacket,
        "set player move flags" SetPlayerMoveFlags  => set_player_move_flags  ::C2SPlaySetPlayerMoveFlagsPacket,
        "loaded"                Loaded              => loaded                 ::C2SPlayLoadedPacket,
        "pong"                  Pong                => pong                   ::C2SPlayPongPacket,
        "select hotbar"         SelectHotbar        => select_hotbar          ::S2CPlaySelectHotbarPacket,
        "swing arm"             SwingArm            => swing_arm              ::C2SPlaySwingArmPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/c2s_play.rs");
