use crate::chunktype::ChunkType;
use crate::error::MidiError;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub struct Track {
    chunk_type: ChunkType,
    length: u32,
    track_event: TrackEvent,
}

impl Track {}

#[derive(Debug, Clone, Copy)]
pub struct TrackEvent {
    v_time: Vql,
    //devo mettere questi tipi di eventi: track_event = <v_time> + <midi_event> | <meta_event> | <sysex_event>
}

//What's VLQ format ?
//It's the Variable-length quantity
//For each byte MSB is set:
// 1 if the next bytes exists
// 0 if it's not exists
// => In the MIDI spec:
// max of 4 bytes
// max value of 2^28-1 = 268435455 = 0x0FFF_FFFF
// Meaning: it's the delta time beetwen THIS track event
// and the previous one.
#[derive(Debug, Clone, Copy)]
pub struct Vql(u32);

impl Vql {
    pub const MAX: u32 = 0x0FFF_FFFF;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn encode(&self) -> [u8; 4] {
        let mut buffer = [0u8; 4];
        let mut value = self.0;

        let mut i = 4;

        loop {
            i -= 1;
            buffer[i] = (value & 0x7F) as u8;
            let be_bytes_from_value = value.to_be_bytes();
            for i in &be_bytes_from_value {
                println!("{:08b}", i);
            }
            value >>= 7;
            if value == 0 {
                break;
            }
        }

        for j in i..3 {
            //set MSB 1
            buffer[j] |= 0x80;
        }

        buffer
    }

    pub fn encode_len(&self) -> usize {
        match self.0 {
            0..=0x7F => 1,
            0x80..=0x3FFF => 2,
            0x4000..=0x1FFFFF => 3,
            _ => 4,
        }
    }

    pub fn encode_bytes(&self) -> Vec<u8> {
        let full = self.encode();
        full[(4 - self.encode_len())..].to_vec()
    }
}

impl TryFrom<u32> for Vql {
    type Error = MidiError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Vql::MAX {
            Ok(Self(value))
        } else {
            Err(MidiError::InvalidVqlInput(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_valid_value() {
        let input = 0x123456;
        let vql = Vql::try_from(input);
        assert!(vql.is_ok());
        assert_eq!(vql.unwrap().value(), input);
    }

    #[test]
    fn try_from_invalid_value() {
        let input = Vql::MAX + 1;
        let vql = Vql::try_from(input);
        assert!(vql.is_err());
        match vql {
            Err(MidiError::InvalidVqlInput(v)) => assert_eq!(v, input),
            _ => panic!("Unexpected error variant"),
        }
    }

    #[test]
    fn encode_one_byte() {
        let vql = Vql::try_from(0x7F).unwrap(); // 127
        let bytes = vql.encode_bytes();
        assert_eq!(bytes, vec![0x7F]);
    }

    #[test]
    fn encode_two_bytes() {
        let vql = Vql::try_from(0x80).unwrap(); // 128
        let bytes = vql.encode_bytes();
        assert_eq!(bytes, vec![0x81, 0x00]);
    }

    #[test]
    fn encode_three_bytes() {
        let vql = Vql::try_from(0x4000).unwrap(); // 16384
        let bytes = vql.encode_bytes();
        assert_eq!(bytes, vec![0x81, 0x80, 0x00]);
    }

    #[test]
    fn encode_four_bytes() {
        let vql = Vql::try_from(0x200000).unwrap(); // 2_097_152
        let bytes = vql.encode_bytes();
        assert_eq!(bytes, vec![0x81, 0x80, 0x80, 0x00]);
    }

    #[test]
    fn encode_len_correctness() {
        assert_eq!(Vql::try_from(0x7F).unwrap().encode_len(), 1);
        assert_eq!(Vql::try_from(0x80).unwrap().encode_len(), 2);
        assert_eq!(Vql::try_from(0x4000).unwrap().encode_len(), 3);
        assert_eq!(Vql::try_from(0x200000).unwrap().encode_len(), 4);
    }
}
