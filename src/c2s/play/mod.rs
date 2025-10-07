//! Serverbound play packets.


pub mod acknowledge_teleport;
pub mod query_block_entity_tag;
// TODO: bundle_item_selected
// TODO: change_difficulty
// TODO: acknowledge_message
// TODO: chat_command
// TODO: signed_chat_command
// TODO: chat_message
// TODO: player_session
// TODO: chunk_batch_received
// TODO: client_status
pub mod tick_end;
// TODO: client_info
// TODO: command_suggestions_request
// TODO: acknowledge_config
// TODO: click_container_button
// TODO: click_container
// TODO: close_container
// TODO: change_container_slot_state
pub mod cookie_response;
pub mod custom_payload;
// TODO: debug_sample_subscription
// TODO: edit_book
// TODO: query_entity_tag
// TODO: interact
// TODO: jigsaw_generate
pub mod keep_alive;
// TODO: lock_difficulty
// TODO: set_player_pos
// TODO: set_player_pos_and_rot
// TODO: set_player_player_rot
// TODO: set_player_move_flags
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
// TODO: pong
// TODO: change_recipe_book_settings
// TODO: set_seen_Recipe
// TODO: rename_item
// TODO: resource_pack_response
// TODO: seen_advancements
// TODO: select_trade
// TODO: set_beacon_effect
// TODO: set_hotbar
// TODO: program_command_block
// TODO: program_command_block_minecart
// TODO: set_creative_mode_slot
// TODO: program_jigsaw_block
// TODO: program_structure_block
// TODO: progrma_test_block
// TODO: update_sign
// TODO: swing_arm
// TODO: teleport_to_entity
// TODO: test_instance_block_action
// TODO: use_item_on
// TODO: use_item
// TODO: custom_click_action


super::packet_group!(
    "play" C2SPlayPackets,
    C2SPlayDecodeError,
    {
        "acknowledge teleport"   AcknowledgeTeleport => acknowledge_teleport   ::C2SPlayAcknowledgeTeleportPacket,
        "query block entity tag" QueryBlockEntityTag => query_block_entity_tag ::C2SPlayQueryBlockEntityTagPacket,
        "tick end"               TickEnd             => tick_end               ::C2SPlayTickEndPacket,
        "cookie response"        CookieResponse      => cookie_response        ::C2SPlayCookieResponsePacket,
        "custom payload"         CustomPayload       => custom_payload         ::C2SPlayCustomPayloadPacket,
        "keep alive"             KeepAlive           => keep_alive             ::C2SPlayKeepAlivePacket,
        "loaded"                 Loaded              => loaded                 ::C2SPlayLoadedPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/c2s_play.rs");
