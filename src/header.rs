use crate::chunktype::ChunkType;
use crate::error::MidiError;
#[derive(Debug, Clone, Copy)]
pub struct Header {
    chunk_type: ChunkType, // b"MThd",
    length: u32,
    format: MidiFormat,
    track_count: u16,
    division: i16,
}

impl Header {
    pub const FIXED_HEADER_LENGTH: u32 = 6;
    pub fn new(format: MidiFormat, track_count: u16, division: i16) -> Self {
        Self {
            chunk_type: ChunkType::Header,
            length: Header::FIXED_HEADER_LENGTH,
            format,
            track_count,
            division,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.chunk_type.as_bytes());

        bytes.extend_from_slice(&self.length.to_be_bytes());

        bytes.extend_from_slice(&(self.format as u16).to_be_bytes());

        bytes.extend_from_slice(&self.track_count.to_be_bytes());

        bytes.extend_from_slice(&self.division.to_be_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MidiError> {
        if bytes.len() < 14 {
            return Err(MidiError::InvalidHeaderByte);
        }

        if &bytes[0..4] != ChunkType::Header.as_bytes() {
            return Err(MidiError::InvalidHeaderByte);
        }

        let length = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        if length != Header::FIXED_HEADER_LENGTH {
            return Err(MidiError::InvalidHeaderByte);
        }

        let format_val = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
        let format = MidiFormat::try_from(format_val).map_err(|_| MidiError::InvalidHeaderByte)?;

        let track_count = u16::from_be_bytes(bytes[10..12].try_into().unwrap());
        let division = i16::from_be_bytes(bytes[12..14].try_into().unwrap());

        Ok(Self {
            chunk_type: ChunkType::Header,
            length,
            format,
            track_count,
            division,
        })
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum MidiFormat {
    SingleTrack = 0,
    MultipleTrack = 1,
    MultipleSong = 2,
}

impl TryFrom<u16> for MidiFormat {
    type Error = MidiError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiFormat::SingleTrack),
            1 => Ok(MidiFormat::MultipleTrack),
            2 => Ok(MidiFormat::MultipleSong),
            _ => Err(MidiError::InvalidFormat),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    //stupid testtt!!
    fn chunk_type() {
        assert_eq!(ChunkType::Header.as_bytes(), [0x4D, 0x54, 0x68, 0x64]); // 'MThd'
        assert_eq!(ChunkType::Track.as_bytes(), [0x4D, 0x54, 0x72, 0x6B]); // 'MTrk'
    }

    #[test]
    fn header_maker() {
        let smf_header = Header::new(MidiFormat::SingleTrack, 1, 1);
        // assert_eq!(smf_header.to_bytes(),);
    }
}
