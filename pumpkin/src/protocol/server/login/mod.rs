use crate::protocol::{bytebuf::ByteBuffer, VarInt};

pub struct SLoginStart {
    pub name: String, // 16
    pub uuid: uuid::Uuid,
}

impl SLoginStart {
    pub const PACKET_ID: VarInt = 0x00;

    pub fn read(bytebuf: &mut ByteBuffer) -> Self {
        Self {
            name: bytebuf.get_string_len(16).unwrap(),
            uuid: bytebuf.get_uuid(),
        }
    }
}

pub struct SEncryptionResponse {
    pub shared_secret_length: VarInt,
    pub shared_secret: Vec<u8>,
    pub verify_token_length: VarInt,
    pub verify_token: Vec<u8>,
}

impl SEncryptionResponse {
    pub const PACKET_ID: VarInt = 0x01;

    pub fn read(bytebuf: &mut ByteBuffer) -> Self {
        let shared_secret_length = bytebuf.get_var_int();
        let shared_secret = bytebuf.copy_to_bytes(shared_secret_length as usize);
        let verify_token_length = bytebuf.get_var_int();
        let verify_token = bytebuf.copy_to_bytes(shared_secret_length as usize);
        Self {
            shared_secret_length,
            shared_secret: shared_secret.to_vec(),
            verify_token_length,
            verify_token: verify_token.to_vec(),
        }
    }
}

pub struct SLoginPluginResponse<'a> {
    message_id: VarInt,
    successful: bool,
    data: Option<&'a [u8]>,
}

impl<'a> SLoginPluginResponse<'a> {
    pub const PACKET_ID: VarInt = 0x02;

    pub fn read(bytebuf: &mut ByteBuffer) -> Self {
        Self {
            message_id: bytebuf.get_var_int(),
            successful: bytebuf.get_bool(),
            data: None, // TODO
        }
    }
}

// Acknowledgement to the Login Success packet sent to the server.
pub struct SLoginAcknowledged {
    // empty
}

impl SLoginAcknowledged {
    pub const PACKET_ID: VarInt = 0x03;

    pub fn read(_bytebuf: &mut ByteBuffer) -> Self {
        Self {}
    }
}
