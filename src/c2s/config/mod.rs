pub mod client_info;
pub mod cookie_response;
pub mod plugin_message;
pub mod finish_acknowledged;
pub mod keep_alive;
pub mod pong;
pub mod pack_status;
pub mod known_packs;
pub mod custom_action;


crate::packet_group!{ "c2s" "config" => pub enum C2SConfigPackets<'l> {
    ClientInfo(client_info::C2SConfigClientInfoPacket),
    CookieResponse(cookie_response::C2SConfigCookieResponsePacket<'l>),
    PluginMessage(plugin_message::C2SConfigPluginMessagePacket<'l>),
    FinishAcknowledged(finish_acknowledged::C2SConfigFinishAcknowledgedPacket),
    KeepAlive(keep_alive::C2SConfigKeepAlivePacket),
    Pong(pong::C2SConfigPongPacket),
    PackStatus(pack_status::C2SConfigPackStatusPacket),
    KnownPacks(known_packs::C2SConfigKnownPacksPacket<'l>),
    CustomAction(custom_action::C2SConfigCustomActionPacket<'l>),
} }
