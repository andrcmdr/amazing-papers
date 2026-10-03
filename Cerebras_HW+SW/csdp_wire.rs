use core::convert::TryFrom;

pub const HEADER_LEN: usize = 64;
pub const MAGIC: [u8; 4] = *b"CSDP";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MessageType {
    Hello = 0x0001,
    Welcome = 0x0002,
    Auth = 0x0003,
    Capabilities = 0x0004,
    StreamOpen = 0x0010,
    StreamAccept = 0x0011,
    StreamClose = 0x0012,
    StreamReset = 0x0013,
    TensorOpen = 0x0020,
    TensorReady = 0x0021,
    Data = 0x0030,
    DataAck = 0x0031,
    CreditUpdate = 0x0032,
    TensorFin = 0x0033,
    Nack = 0x0034,
    CollectiveOpen = 0x0040,
    CollectiveReady = 0x0041,
    CollectiveData = 0x0042,
    CollectiveFin = 0x0043,
    Heartbeat = 0x0050,
    Pong = 0x0051,
    Error = 0x00ff,
}

impl TryFrom<u16> for MessageType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0x0001 => Self::Hello,
            0x0002 => Self::Welcome,
            0x0003 => Self::Auth,
            0x0004 => Self::Capabilities,
            0x0010 => Self::StreamOpen,
            0x0011 => Self::StreamAccept,
            0x0012 => Self::StreamClose,
            0x0013 => Self::StreamReset,
            0x0020 => Self::TensorOpen,
            0x0021 => Self::TensorReady,
            0x0030 => Self::Data,
            0x0031 => Self::DataAck,
            0x0032 => Self::CreditUpdate,
            0x0033 => Self::TensorFin,
            0x0034 => Self::Nack,
            0x0040 => Self::CollectiveOpen,
            0x0041 => Self::CollectiveReady,
            0x0042 => Self::CollectiveData,
            0x0043 => Self::CollectiveFin,
            0x0050 => Self::Heartbeat,
            0x0051 => Self::Pong,
            0x00ff => Self::Error,
            _ => return Err("unknown message type"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FrameHeader {
    pub version_major: u8,
    pub version_minor: u8,
    pub header_len_words: u8,
    pub flags: u8,
    pub message_type: MessageType,
    pub status: u16,
    pub session_id: u64,
    pub stream_id: u32,
    pub sequence: u64,
    pub object_id: u64,
    pub chunk_id: u32,
    pub payload_len: u64,
    pub payload_crc32c: u32,
}

impl FrameHeader {
    pub fn new(
        message_type: MessageType,
        session_id: u64,
        stream_id: u32,
        sequence: u64,
        object_id: u64,
        chunk_id: u32,
        payload: &[u8],
    ) -> Self {
        Self {
            version_major: 1,
            version_minor: 0,
            header_len_words: 8,
            flags: 0,
            message_type,
            status: 0,
            session_id,
            stream_id,
            sequence,
            object_id,
            chunk_id,
            payload_len: payload.len() as u64,
            payload_crc32c: crc32c(payload),
        }
    }

    pub fn encode(&self) -> [u8; HEADER_LEN] {
        let mut out = [0u8; HEADER_LEN];
        out[0..4].copy_from_slice(&MAGIC);
        out[4] = self.version_major;
        out[5] = self.version_minor;
        out[6] = self.header_len_words;
        out[7] = self.flags;
        out[8..10].copy_from_slice(&(self.message_type as u16).to_le_bytes());
        out[10..12].copy_from_slice(&self.status.to_le_bytes());
        out[12..20].copy_from_slice(&self.session_id.to_le_bytes());
        out[20..24].copy_from_slice(&self.stream_id.to_le_bytes());
        out[24..32].copy_from_slice(&self.sequence.to_le_bytes());
        out[32..40].copy_from_slice(&self.object_id.to_le_bytes());
        out[40..44].copy_from_slice(&self.chunk_id.to_le_bytes());
        out[44..48].fill(0);
        out[48..56].copy_from_slice(&self.payload_len.to_le_bytes());
        out[56..60].copy_from_slice(&self.payload_crc32c.to_le_bytes());
        out[60..64].fill(0);
        let checksum = crc32c(&out[..60]);
        out[60..64].copy_from_slice(&checksum.to_le_bytes());
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < HEADER_LEN {
            return Err("short header");
        }
        if bytes[0..4] != MAGIC {
            return Err("bad magic");
        }
        if bytes[6] < 8 {
            return Err("invalid header length");
        }

        let mut tmp = [0u8; HEADER_LEN];
        tmp.copy_from_slice(&bytes[..HEADER_LEN]);
        let expected = u32::from_le_bytes(tmp[60..64].try_into().unwrap());
        tmp[60..64].fill(0);
        if crc32c(&tmp[..60]) != expected {
            return Err("header checksum failure");
        }

        let message_type = MessageType::try_from(u16::from_le_bytes(
            bytes[8..10].try_into().unwrap(),
        ))
        .map_err(|_| "unknown message type")?;

        Ok(Self {
            version_major: bytes[4],
            version_minor: bytes[5],
            header_len_words: bytes[6],
            flags: bytes[7],
            message_type,
            status: u16::from_le_bytes(bytes[10..12].try_into().unwrap()),
            session_id: u64::from_le_bytes(bytes[12..20].try_into().unwrap()),
            stream_id: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            sequence: u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
            object_id: u64::from_le_bytes(bytes[32..40].try_into().unwrap()),
            chunk_id: u32::from_le_bytes(bytes[40..44].try_into().unwrap()),
            payload_len: u64::from_le_bytes(bytes[48..56].try_into().unwrap()),
            payload_crc32c: u32::from_le_bytes(bytes[56..60].try_into().unwrap()),
        })
    }

    pub fn verify_payload(&self, payload: &[u8]) -> Result<(), &'static str> {
        if payload.len() as u64 != self.payload_len {
            return Err("payload length mismatch");
        }
        if crc32c(payload) != self.payload_crc32c {
            return Err("payload checksum failure");
        }
        Ok(())
    }
}

/// Portable, dependency-free CRC32C implementation for the reference module.
/// Production implementations SHOULD use a hardware-accelerated implementation.
pub fn crc32c(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0x82f6_3b78u32 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_round_trip() {
        let payload = b"HELLO-CSDP";
        let header = FrameHeader::new(
            MessageType::Data,
            0x1122_3344_5566_7788,
            7,
            1,
            0xAABB_CCDD_EEFF_0011,
            3,
            payload,
        );
        let wire = header.encode();
        let decoded = FrameHeader::decode(&wire).unwrap();
        decoded.verify_payload(payload).unwrap();
        assert_eq!(decoded.object_id, 0xAABB_CCDD_EEFF_0011);
        assert_eq!(decoded.chunk_id, 3);
        assert_eq!(decoded.payload_len, payload.len() as u64);
    }
}
