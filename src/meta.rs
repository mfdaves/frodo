use crate::track::Vql;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaEvent {
    TrackName(Vec<u8>),
    EndOfTrack,
    SetTempo(u32),
    TimeSignature {
        numerator: u8,
        denominator: u8,
        clocks_per_tick: u8,
        thirty_seconds_per_24_clocks: u8,
    },
    KeySignature {
        sharps: i8,
        is_major: bool,
    },
    Unknown {
        event_type: u8,
        data: Vec<u8>,
    },
}

impl MetaEvent {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0xFF]; // Meta event marker

        let (type_byte, data) = match self {
            MetaEvent::TrackName(d) => (0x03, d.clone()),
            MetaEvent::EndOfTrack => (0x2F, vec![]),
            MetaEvent::SetTempo(t) => (0x51, t.to_be_bytes()[1..].to_vec()),
            MetaEvent::TimeSignature {
                numerator,
                denominator,
                clocks_per_tick,
                thirty_seconds_per_24_clocks,
            } => (
                0x58,
                vec![
                    *numerator,
                    (*denominator as f32).log2() as u8,
                    *clocks_per_tick,
                    *thirty_seconds_per_24_clocks,
                ],
            ),
            MetaEvent::KeySignature { sharps, is_major } => {
                (0x59, vec![*sharps as u8, if *is_major { 0 } else { 1 }])
            }
            MetaEvent::Unknown { event_type, data } => (*event_type, data.clone()),
        };

        bytes.push(type_byte);
        let len_vql = Vql::try_from(data.len() as u32).unwrap();
        bytes.extend(len_vql.encode_bytes());
        bytes.extend(data);

        bytes
    }
}
