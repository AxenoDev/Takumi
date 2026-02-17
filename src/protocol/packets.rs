use tracing_subscriber::fmt::format;
use uuid::Uuid;

use crate::protocol::{
    Packet,
    types::{write_byte_array, write_string, write_uuid, write_varint, write_varlong},
};

// --- Serverbound Handshaking ---
pub const SB_HANDSHAKE: i32 = 0x00;

// --- Serverbound Status ---
pub const SB_STATUS_REQUEST: i32 = 0x00;
pub const SB_STATUS_PING: i32 = 0x01;

// --- Serverbound Login ---
pub const SB_LOGIN_START: i32 = 0x00;
pub const SB_LOGIN_ENCRYPTION_RESPONSE: i32 = 0x01;
pub const SB_LOGIN_PLUGIN_RESPONSE: i32 = 0x02;
pub const SB_LOGIN_ACK: i32 = 0x03;

// --- Clientbound Status ---
pub const CB_STATUS_RESPONSE: i32 = 0x00;
pub const CB_STATUS_PONG: i32 = 0x01;

// --- Clientbound Login ---
pub const CB_LOGIN_DISCONNECT: i32 = 0x00;
pub const CB_LOGIN_ENCRYPTION_REQUEST: i32 = 0x01;
pub const CB_LOGIN_SUCCESS: i32 = 0x02;
pub const CB_LOGIN_SET_COMPRESSION: i32 = 0x03;
pub const CB_LOGIN_PLUGIN_REQUEST: i32 = 0x04;

// -- Clientbound Play ---
pub const CB_PLAY_DISCONNECT: i32 = 0x1A;

// Builders
pub fn build_status_response(json: &str) -> Packet {
    Packet::build(CB_STATUS_RESPONSE, |buf| {
        write_string(buf, json);
    })
}

pub fn build_status_pong(payload: i64) -> Packet {
    Packet::build(CB_STATUS_PONG, |buf| {
        write_varlong(buf, payload);
    })
}

pub fn build_login_disconnect(reason: &str) -> Packet {
    Packet::build(CB_LOGIN_DISCONNECT, |buf| {
        write_string(buf, &format!("{{\"text\":\"{reason}\"}}"));
    })
}

pub fn build_encryption_request(server_id: &str, public_key: &[u8], verify_token: &[u8]) -> Packet {
    Packet::build(CB_LOGIN_ENCRYPTION_REQUEST, |buf| {
        write_string(buf, server_id);
        write_byte_array(buf, public_key);
        write_byte_array(buf, verify_token);
    })
}

pub fn build_login_success(uuid: &Uuid, username: &str) -> Packet {
    Packet::build(CB_LOGIN_SUCCESS, |buf| {
        write_uuid(buf, uuid);
        write_string(buf, username);
        write_varint(buf, 0); // value is hardcoded to 0 in this context
    })
}

pub fn build_set_compression(threshold: i32) -> Packet {
    Packet::build(CB_LOGIN_SET_COMPRESSION, |buf| {
        write_varint(buf, threshold);
    })
}

pub fn build_login_plugin_request(message_id: i32, channel: &str, data: &[u8]) -> Packet {
    Packet::build(CB_LOGIN_PLUGIN_REQUEST, |buf| {
        write_varint(buf, message_id);
        write_string(buf, channel);
        buf.extend_from_slice(data);
    })
}

pub fn build_play_disconnect(reason: &str) -> Packet {
    Packet::build(CB_PLAY_DISCONNECT, |buf| {
        write_string(buf, &format!("{{\"text\":\"{reason}\"}}"));
    })
}
