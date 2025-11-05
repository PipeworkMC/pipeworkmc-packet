pub mod start;
pub mod encrypt_response;
pub mod plugin_response;
pub mod finish_acknowledged;
pub mod cookie_response;


crate::packet_group!{ "c2s" "login" => pub enum C2SLoginPackets<'l> {
    Start(start::C2SLoginStartPacket),
    EncryptResponse(encrypt_response::C2SLoginEncryptResponsePacket<'l>),
    PluginResponse(plugin_response::C2SLoginPluginResponsePacket<'l>),
    FinishAcknowledged(finish_acknowledged::C2SLoginFinishAcknowledgedPacket),
    CookieResponse(cookie_response::C2SLoginCookieResponsePacket<'l>)
} }
