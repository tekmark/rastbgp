use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io;

#[derive(Debug)]
pub enum BgpMessage {
    Open,
    Keepalive,
    Notification,
    Update,
}

impl BgpMessage {
    pub fn parse(mut buf: BytesMut) -> io::Result<Self> {
        // BGP header: 16-byte marker, 2-byte length, 1-byte type
        if buf.len() < 19 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "incomplete BGP header"));
        }

        buf.advance(16); // skip marker
        let _length = buf.get_u16();
        let msg_type = buf.get_u8();

        match msg_type {
            1 => Ok(BgpMessage::Open),
            2 => Ok(BgpMessage::Update),
            3 => Ok(BgpMessage::Notification),
            4 => Ok(BgpMessage::Keepalive),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "unknown BGP message type")),
        }
    }
    pub fn encode(&self) -> Bytes {
        match self {
            BgpMessage::Keepalive => {
                let mut buf = BytesMut::with_capacity(19);
                buf.extend_from_slice(&[0xFF; 16]); // Marker
                buf.put_u16(19); // Length
                buf.put_u8(4); // Type = KEEPALIVE
                buf.freeze()
            }
            BgpMessage::Open => {
                let mut buf = BytesMut::with_capacity(29);
                buf.extend_from_slice(&[0xFF; 16]);
                buf.put_u16(29);
                buf.put_u8(1); // Type = OPEN
                buf.put_u8(4); // Version
                buf.put_u16(64512); // My ASN (mock)
                buf.put_u16(90); // Hold Time
                buf.put_u32(0xC0A80101); // BGP Identifier (192.168.1.1)
                buf.put_u8(0); // Optional Params Length
                buf.freeze()
            }
            _ => todo!("Encoding for {:?} not implemented", self),
        }
    }
}
