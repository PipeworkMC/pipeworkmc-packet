//! Serverbound configuration packets.


pub mod client_info;
// TODO: cookie_response
pub mod custom_payload;
pub mod finish_acknowledged;
pub mod keep_alive;
// TODO: pong
// TODO: resource_pack_response
pub mod known_packs;
// TODO: custom_click_action


super::packet_group!(
    "configuration" C2SConfigPackets,
    C2SConfigDecodeError,
    {
        "client info"         ClientInfo         => client_info         ::C2SConfigClientInfoPacket,
        "custom payload"      CustomPayload      => custom_payload      ::C2SConfigCustomPayloadPacket,
        "finish acknowledged" FinishAcknowledged => finish_acknowledged ::C2SConfigFinishAcknowledgedPacket,
        "keep alive"          KeepAlive          => keep_alive          ::C2SConfigKeepAlivePacket,
        "known packs"         KnownPacks         => known_packs         ::C2SConfigKnownPacksPacket
    }
);
