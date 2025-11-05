pub mod disconnect;
pub mod encrypt_request;
pub mod finished;
pub mod compression;
pub mod plugin_request;
pub mod cookie_request;



crate::packet_group!{ "s2c" "login" => pub enum S2CLoginPackets<'l> {
    Disconnect(disconnect::S2CLoginDisconnectPacket<'l>),
    EncryptRequest(encrypt_request::S2CLoginEncryptRequestPacket<'l>),
    Finished(finished::S2CLoginFinishedPacket),
    Compression(compression::S2CLoginCompressionPacket),
    CookieRequest(cookie_request::C2SLoginCookieRequestPacket)
} }
