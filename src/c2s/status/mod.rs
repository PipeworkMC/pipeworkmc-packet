pub mod request;
pub mod ping;


crate::packet_group!{ "c2s" "status" => pub enum C2SStatusPackets {
    Request(request::C2SStatusRequestPacket),
    Ping(ping::C2SStatusPingPacket)
} }
