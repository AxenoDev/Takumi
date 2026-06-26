use crate::connection::Connection;
use crate::error::ProtocolError;
use crate::packet::RawPacket;
use crate::packets::login::cookie_response_login::CookieResponseLoginPacket;
use crate::packets::login::encryption_response::EncryptionResponsePacket;
use crate::packets::login::login_acknowledged::LoginAcknowledgedPacket;
use crate::packets::login::login_plugin_response::LoginPluginResponsePacket;
use crate::packets::login::login_start::LoginStartPacket;
use crate::packets::login::login_success::LoginSuccessPacket;
use crate::{ConnectionState, PacketMeta};

pub mod cookie_response_login;
pub mod encryption_response;
pub mod login_acknowledged;
pub mod login_plugin_response;
pub mod login_start;
pub mod login_success;

pub async fn handle(
    conn: &mut Connection,
    raw: RawPacket,
) -> Result<Option<ConnectionState>, ProtocolError> {
    match raw.id {
        LoginStartPacket::ID => {
            let login: LoginStartPacket = raw.decode()?;
            println!("LoginStartPacket: name={}, uuid={}", login.name, login.uuid);
            conn.send(&LoginSuccessPacket::offline(
                login.uuid,
                login.name.clone(),
                776,
            ))
            .await?;
            Ok(None)
        }

        LoginAcknowledgedPacket::ID => {
            let _: LoginAcknowledgedPacket = raw.decode()?;
            println!("LoginAcknowledgedPacket received → switching to Configuration");
            Ok(Some(ConnectionState::Configuration))
        }

        EncryptionResponsePacket::ID => {
            let _: EncryptionResponsePacket = raw.decode()?;
            println!("EncryptionResponsePacket received");
            Ok(None)
        }

        LoginPluginResponsePacket::ID => {
            let p: LoginPluginResponsePacket = raw.decode()?;
            println!(
                "LoginPluginResponsePacket: message_id={}, data={:?}",
                p.message_id, p.data
            );
            Ok(None)
        }

        CookieResponseLoginPacket::ID => {
            let p: CookieResponseLoginPacket = raw.decode()?;
            println!(
                "CookieResponseLoginPacket: key={}, payload={:?}",
                p.key, p.payload
            );
            Ok(None)
        }

        id => Err(ProtocolError::UnknownPacket {
            id,
            conn: Some(ConnectionState::Login),
        }),
    }
}
