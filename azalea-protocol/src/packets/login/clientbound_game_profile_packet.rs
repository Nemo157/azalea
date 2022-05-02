use std::io::Read;

use super::LoginPacket;
use crate::mc_buf::{Readable, Writable};
use azalea_auth::game_profile::GameProfile;
use azalea_core::serializable_uuid::SerializableUuid;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ClientboundGameProfilePacket {
    pub game_profile: GameProfile,
}

// TODO: add derives to GameProfile and have an impl McBufReadable/Writable for GameProfile
impl ClientboundGameProfilePacket {
    pub fn get(self) -> LoginPacket {
        LoginPacket::ClientboundGameProfilePacket(self)
    }

    pub fn write(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        for n in self.game_profile.uuid.to_int_array() {
            buf.write_int(n as i32).unwrap();
        }
        buf.write_utf(self.game_profile.name.as_str()).unwrap();
        Ok(())
    }

    pub fn read(buf: &mut impl Read) -> Result<LoginPacket, String> {
        // TODO: we have a thing to read from the uuid now
        let uuid = Uuid::from_int_array([
            buf.read_int()? as u32,
            buf.read_int()? as u32,
            buf.read_int()? as u32,
            buf.read_int()? as u32,
        ]);
        let name = buf.read_utf_with_len(16)?;
        Ok(ClientboundGameProfilePacket {
            game_profile: GameProfile::new(uuid, name),
        }
        .get())
    }
}
