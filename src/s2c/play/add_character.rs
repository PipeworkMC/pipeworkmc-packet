use crate::s2c::{
    S2CPackets,
    play::S2CPlayPackets
};
use pipeworkmc_codec::{
    encode::{
        PacketEncode,
        EncodeBuf
    },
    meta::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use pipeworkmc_data::{
    angle::Angle,
    character::{
        CharacterId,
        CharacterType,
        CharacterPos,
        CharacterRot,
        CharacterVel
    },
    uuid::Uuid,
    varint::VarInt
};


#[derive(Debug)]
pub struct S2CPlayAddCharacterPacket {
    pub eid  : CharacterId,
    pub uuid : Uuid,
    pub ty   : CharacterType,
    pub pos  : CharacterPos,
    pub rot  : CharacterRot,
    pub data : u32,
    pub vel  : CharacterVel
}

impl PacketMeta for S2CPlayAddCharacterPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayAddCharacterPacket {

    fn encode_len(&self) -> usize {
        VarInt::<u32>(self.eid.as_u32()).encode_len()
        + self.uuid.encode_len()
        + VarInt::<u32>(self.ty.protocol_id()).encode_len()
        + self.pos.x.encode_len()
        + self.pos.y.encode_len()
        + self.pos.z.encode_len()
        + Angle::radians(self.rot.pitch).encode_len()
        + Angle::radians(self.rot.yaw).encode_len()
        + Angle::radians(self.rot.head_yaw).encode_len()
        + VarInt::<u32>(self.data).encode_len()
        + ((self.vel.x * 8000.0) as i16).encode_len()
        + ((self.vel.y * 8000.0) as i16).encode_len()
        + ((self.vel.z * 8000.0) as i16).encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        VarInt::<u32>(self.eid.as_u32()).encode(buf);
        self.uuid.encode(buf);
        VarInt::<u32>(self.ty.protocol_id()).encode(buf);
        self.pos.x.encode(buf);
        self.pos.y.encode(buf);
        self.pos.z.encode(buf);
        Angle::radians(self.rot.pitch).encode(buf);
        Angle::radians(self.rot.yaw).encode(buf);
        Angle::radians(self.rot.head_yaw).encode(buf);
        VarInt::<u32>(self.data).encode(buf);
        ((self.vel.x * 8000.0) as i16).encode(buf);
        ((self.vel.y * 8000.0) as i16).encode(buf);
        ((self.vel.z * 8000.0) as i16).encode(buf);
    } }

}

impl From<S2CPlayAddCharacterPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayAddCharacterPacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayAddCharacterPacket> for S2CPlayPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayAddCharacterPacket) -> Self { Self::AddCharacter(value) }
}
