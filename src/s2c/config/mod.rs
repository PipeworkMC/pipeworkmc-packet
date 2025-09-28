//! Clientbound configuration packets.


// TODO: cookie_request
pub mod custom_payload;
pub mod disconnect;
pub mod finish;
pub mod keep_alive;
// TODO: ping
// TODO: reset_chat
pub mod registry_data;
// TODO: remove_resource_pack
// TODO: add_resource_pack
// TODO: store_cookie
// TODO: transfer
// TODO: feature_flags
// TODO: update_tags
pub mod known_packs;
// TODO: custom_report_details
// TODO: server_links
// TODO: clear_dialog
// TODO: show_dialog


super::packet_group!(
    "configuration" Config S2CConfigPackets<'l>, meta,
    {
        "custom payload" CustomPayload => custom_payload ::S2CConfigCustomPayloadPacket<'l>,
        "disconnect"     Disconnect    => disconnect     ::S2CConfigDisconnectPacket,
        "finish"         Finish        => finish         ::S2CConfigFinishPacket,
        "keep_alive"     KeepAlive     => keep_alive     ::S2CConfigKeepAlivePacket,
        "registry_data"  RegistryData  => registry_data  ::S2CConfigRegistryDataPacket<'l>,
        "known_packs"    KnownPacks    => known_packs    ::S2CConfigKnownPacksPacket<'l>
    }
);
