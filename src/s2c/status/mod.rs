pub mod response;
pub mod pong;


crate::packet_group!{ "s2c" "status" => pub enum S2CStatusPackets<'l> {
    Response(response::S2CStatusResponsePacket<'l>),
    Pong(pong::S2CStatusPongPacket)
} }
