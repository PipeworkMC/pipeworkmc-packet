pub mod intent;


crate::packet_group!{ "c2s" "handshake" => pub enum C2SHandshakePackets<'l> {
    Intent(intent::C2SHandshakeIntentPacket<'l>)
} }
