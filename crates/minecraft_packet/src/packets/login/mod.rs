use crate::PacketMeta;
use crate::connection::Connection;
use crate::error::ProtocolError;
use crate::packet::RawPacket;
use crate::packets::login::cookie_response_login::CookieResponseLoginPacket;
use crate::packets::login::encryption_response::EncryptionResponsePacket;
use crate::packets::login::login_acknowledged::LoginAcknowledgedPacket;
use crate::packets::login::login_plugin_response::LoginPluginResponsePacket;
use crate::packets::login::login_start::LoginStartPacket;
use crate::packets::login::login_success::LoginSuccessPacket;

pub mod cookie_response_login;
pub mod encryption_response;
pub mod login_acknowledged;
pub mod login_plugin_response;
pub mod login_start;
pub mod login_success;

pub async fn handle(conn: &mut Connection, raw: RawPacket) -> Result<(), ProtocolError> {
    match raw.id {
        LoginStartPacket::ID => {
            let login: LoginStartPacket = raw.decode()?;

            println!("LoginStartPacket: name={}, uuid={}", login.name, login.uuid);

            conn.send(&LoginSuccessPacket::offline(login.uuid, login.name.clone(), 776)).await?;

            Ok(())
        }

        EncryptionResponsePacket::ID => {
            let _encryption_response: EncryptionResponsePacket = raw.decode()?;
            println!("EncryptionResponsePacket received");
            Ok(())
        }

        LoginPluginResponsePacket::ID => {
            let plugin_response: LoginPluginResponsePacket = raw.decode()?;

            println!(
                "LoginPluginResponsePacket: message_id={}, data={:?}",
                plugin_response.message_id, plugin_response.data
            );

            Ok(())
        }

        LoginAcknowledgedPacket::ID => {
            let _login_acknowledged: LoginAcknowledgedPacket = raw.decode()?;
            println!("LoginAcknowledgedPacket received");
            Ok(())
        }

        CookieResponseLoginPacket::ID => {
            let cookie_response: CookieResponseLoginPacket = raw.decode()?;
            println!(
                "CookieResponseLoginPacket: key={}, payload={:?}",
                cookie_response.key, cookie_response.payload
            );
            Ok(())
        }

        id => Err(ProtocolError::UnknownPacket { id }),
    }
}
