use crate::{OutgoingPacket, error::ProtocolError, writer::PacketWriter};
use takumi_macros::PacketOut;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PacketOut)]
#[packet(id = 0x02)]
pub struct LoginSuccessPacket {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
    pub protocol_version: i32,
}

impl LoginSuccessPacket {
    pub fn offline(uuid: Uuid, username: String, protocol_version: i32) -> Self {
        Self {
            uuid,
            username,
            properties: vec![],
            protocol_version,
        }
    }
}

impl OutgoingPacket for LoginSuccessPacket {
    fn encode_payload(&self, writer: &mut PacketWriter) -> Result<(), ProtocolError> {
        writer.write_uuid(&self.uuid);
        writer.write_string(&self.username);

        writer.write_varint(self.properties.len() as i32);
        for prop in &self.properties {
            writer.write_string(&prop.name);
            writer.write_string(&prop.value);
            match &prop.signature {
                Some(sig) => {
                    writer.write_bool(true);
                    writer.write_string(sig);
                }
                None => {
                    writer.write_bool(false);
                }
            }
        }

        if self.protocol_version >= 776 {
            writer.write_uuid(&Uuid::new_v4());
        }

        println!(
            "LoginSuccessPacket sent: uuid={}, username={}, properties={:?}",
            self.uuid, self.username, self.properties
        );

        Ok(())
    }
}
