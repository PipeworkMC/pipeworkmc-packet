use crate::meta::*;
use pipeworkmc_data::Minecraft;
use std::borrow::Cow;
use netzer::prelude::*;


#[derive(Clone, Debug, NetEncode, NetDecode)]
pub struct C2SHandshakeIntentPacket<'l> {
    #[netzer(convert = "VarInt<u32>")]
    pub protocol : u32,
    pub address  : Cow<'l, str>,
    pub port     : u16,
    pub intent   : Intent
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Intent {
    Status,
    Login {
        transfer : bool
    }
}
impl NetEncode<Minecraft> for Intent {
    async fn encode<W : netzer::AsyncWrite>(&self, mut w : W) -> netzer::Result {
        let x = (match (self) {
            Intent::Status                     => 1,
            Intent::Login { transfer : false } => 2,
            Intent::Login { transfer : true  } => 3
        });
        w.write(&[x]).await?;
        Ok(())
    }
}
impl NetDecode<Minecraft> for Intent {
    async fn decode<R : netzer::AsyncRead>(mut r : R) -> netzer::Result<Self> {
        let mut buf = [0u8; 1];
        r.read(&mut buf).await?;
        Ok(match (buf[0]) {
            1 => Intent::Status,
            2 => Intent::Login { transfer : false },
            3 => Intent::Login { transfer : true  },
            x => {
                return Err(format!("invalid intent {x}").into());
            }
        })
    }
}


impl Packet for C2SHandshakeIntentPacket<'_> {
    const PREFIX : u8 = 0; // TODO: Use datagen.
    type Bound = Bound::C2S;
    type State = State::Handshake;
}
