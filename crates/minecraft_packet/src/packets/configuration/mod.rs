pub mod acknowledge_finish_configuration;
pub mod client_information;
pub mod finish_configuration;
pub mod plugin_message;

use crate::ConnectionState;
use crate::PacketMeta;
use crate::connection::Connection;
use crate::error::ProtocolError;
use crate::packet::RawPacket;

use acknowledge_finish_configuration::AcknowledgeFinishConfigurationPacket;
use client_information::ClientInformationPacket;
use plugin_message::PluginMessagePacket;

pub async fn handle(
    _conn: &mut Connection,
    raw: RawPacket,
) -> Result<Option<ConnectionState>, ProtocolError> {
    match raw.id {
        ClientInformationPacket::ID => {
            let info: ClientInformationPacket = raw.decode()?;
            println!(
                "ClientInformation: locale={}, view_distance={}",
                info.locale, info.view_distance
            );
            Ok(None)
        }

        PluginMessagePacket::ID => {
            let msg: PluginMessagePacket = raw.decode()?;
            println!("PluginMessage (config): channel={}", msg.channel);
            Ok(None)
        }

        AcknowledgeFinishConfigurationPacket::ID => {
            let _: AcknowledgeFinishConfigurationPacket = raw.decode()?;
            println!("AcknowledgeFinishConfiguration → entering Play");
            Ok(Some(ConnectionState::Play))
        }

        id => Err(ProtocolError::UnknownPacket {
            id,
            conn: Some(ConnectionState::Configuration),
        }),
    }
}
