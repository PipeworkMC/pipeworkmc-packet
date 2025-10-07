//! Serverbound configuration packets.


pub mod client_info;
pub mod cookie_response;
pub mod custom_payload;
pub mod finish_acknowledged;
pub mod keep_alive;
pub mod pong;
pub mod resource_pack_status;
pub mod known_packs;
// pub mod custom_click_action; // TODO


super::packet_group!(
    "configuration" C2SConfigPackets,
    C2SConfigDecodeError,
    {
        "client info"          ClientInfo         => client_info          ::C2SConfigClientInfoPacket,
        "cookie response"      CookieResponse     => cookie_response      ::C2SConfigCookieResponsePacket,
        "custom payload"       CustomPayload      => custom_payload       ::C2SConfigCustomPayloadPacket,
        "finish acknowledged"  FinishAcknowledged => finish_acknowledged  ::C2SConfigFinishAcknowledgedPacket,
        "keep alive"           KeepAlive          => keep_alive           ::C2SConfigKeepAlivePacket,
        "pong"                 Pong               => pong                 ::C2SConfigPongPacket,
        "resource pack status" ResourcePackStatus => resource_pack_status ::C2SConfigResourcePackStatusPacket,
        "known packs"          KnownPacks         => known_packs          ::C2SConfigKnownPacksPacket,
        // "custom click action"  CustomClickAction  => custom_click_action  ::C2SConfigCustomClickActionPacket
    }
);


include!("../../../../pipeworkmc-vanilla-datagen/output/generated/packet/c2s_config.rs");
