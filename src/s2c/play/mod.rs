//! Clientbound play packets.


// TODO: bundle_delimiter
pub mod add_character;
// TODO: character_animation
// TODO: award_statistics
// TODO: acknowledge_block_change
// TODO: set_block_destroy_stage
// TODO: block_data
// TODO: block_action
// TODO: block_update
// TODO: boss_bar
// TODO: change_difficulty
// TODO: chunk_batch_finished
// TODO: chunk_batch_start
// TODO: chunk_biomes
// TODO: clear_titles
// TODO: command_suggestions_response
// TODO: commands
// TODO: close_container
// TODO: set_container_content
// TODO: set_container_property
// TODO: set_container_slot
// TODO: cookie_request
// TODO: set_cooldown
// TODO: chat_suggestions
// TODO: custom_payload
// TODO: damage_event
// TODO: debug_sample
// TODO: delete_message
pub mod disconnect;
// TODO: disguised_chat_message
// TODO: character_event
// TODO: teleport_character
// TODO: explosion
// TODO: unload_chunk
pub mod game_event;
// TODO: open_horse_screen
// TODO: hurt_animation
// TODO: initialise_world_border
pub mod keep_alive;
// TODO: chunk_data_and_light
// TODO: world_event
// TODO: particle
// TODO: update_light
pub mod login;
// TODO: map_data
// TODO: merchant_offers
// TODO: update_character_pos
// TODO: update_character_pos_and_rot
// TODO: move_minecart_along_track
// TODO: update_character_rot
// TODO: move_vehicle
// TODO: open_book
// TODO: open_screen
// TODO: open_sign_editor
// TODO: ping
// TODO: pong
// TODO: place_ghost_recipe
// TODO: player_abilities
// TODO: player_chat_message
// TODO: end_combat
// TODO: enter_combat
// TODO: combat_death
// TODO: player_info_remove
// TODO: player_info_update
// TODO: look_at
// TODO: synchronise_player_pos
// TODO: player_rot
// TODO: recipe_book_add
// TODO: recipe_book_remove
// TODO: recipe_book_settings
pub mod remove_characters;
// TODO: remove_character_effect
// TODO: reset_score
// TODO: remove_resource_pack
// TODO: add_resource_pack
// TODO: respawn
// TODO: set_head_rot
// TODO: update_section_blocks
// TODO: select_advancements_tab
// TODO: server_data
// TODO: action_bar
// TODO: set_border_centre
// TODO: set_border_lerp_size
// TODO: set_border_size
// TODO: set_border_warning_delay
// TODO: set_border_warning_distance
// TODO: set_camera
// TODO: set_centre_chunk
// TODO: set_render_distance
// TODO: set_cursor_item
// TODO: set_default_spawn_pos
// TODO: display_objective
// TODO: set_character_metadata
// TODO: link_characters
// TODO: set_character_velocity
// TODO: set_equipment
// TODO: set_experience
// TODO: set_health
// TODO: set_hotbar
// TODO: update_objectives
// TODO: set_passengers
// TODO: set_player_inventory_slot
// TODO: update_teams
// TODO: update_score
// TODO: set_simulation_distance
// TODO: set_subtitle_text
// TODO: update_time
// TODO: set_title_text
// TODO: set_title_animation_times
// TODO: character_sound_effect
// TODO: sound_effect
// TODO: start_config
// TODO: stop_sound
// TODO: store_cookie
// TODO: system_chat_message
// TODO: set_tab_list_header
// TODO: tag_query_response
// TODO: pickup_item
// TODO: synchronise_vehicle_pos
// TODO: test_instance_block_status
// TODO: set_ticking_State
// TODO: step_tick
// TODO: transfer
// TODO: update_advancements
// TODO: update_attributes
// TODO: character_effect
// TODO: update_recipe
// TODO: update_tags
// TODO: projectile_power
// TODO: custom_report_details
// TODO: server_links
// TODO: waypoint
// TODO: clear_dialog
// TODO: show_dialog


super::packet_group!(
    "play" Play S2CPlayPackets<'l>, meta,
    {
        "add_character"     AddCharacter     => add_character     ::S2CPlayAddCharacterPacket,
        "disconnect"        Disconnect       => disconnect        ::S2CPlayDisconnectPacket,
        "game_event"        GameEvent        => game_event        ::S2CPlayGameEventPacket,
        "keep_alive"        KeepAlive        => keep_alive        ::S2CPlayKeepAlivePacket,
        "login"             Login            => login             ::S2CPlayLoginPacket<'l>,
        "remove_characters" RemoveCharacters => remove_characters ::S2CPlayRemoveCharactersPacket<'l>
    }
);
