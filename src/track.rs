use crate::chunktype::ChunkType;
use crate::error::MidiError;
use crate::event::MidiEvent;
use crate::message::{ChannelMessage, MidiMessage, Status};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Track {
    chunk_type: ChunkType,
    #[allow(dead_code)]
    length: u32,
    events: Vec<TrackEvent>,
}

impl Track {
    pub fn new(events: Vec<TrackEvent>) -> Self {
        Self {
            chunk_type: ChunkType::Track,
            length: 0, // Will be calculated in to_bytes
            events,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut track_data = Vec::new();
        for event in &self.events {
            track_data.extend(event.to_bytes());
        }

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.chunk_type.as_bytes());
        bytes.extend_from_slice(&(track_data.len() as u32).to_be_bytes());
        bytes.extend(track_data);

        bytes
    }
}

#[derive(Debug, Clone)]
pub enum EventType {
    Midi(MidiMessage),
    Meta(crate::meta::MetaEvent),
    Sysex(Vec<u8>),
}

impl EventType {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            EventType::Midi(msg) => {
                let mut bytes = Vec::new();
                match msg {
                    MidiMessage::Channel { channel, message } => {
                        let event = match message {
                            ChannelMessage::NoteOff { .. } => MidiEvent::NoteOff,
                            ChannelMessage::NoteOn { .. } => MidiEvent::NoteOn,
                            ChannelMessage::PolyphonicKeyPressure { .. } => {
                                MidiEvent::PolyphonicKeyPressure
                            }
                            ChannelMessage::ControlChange { .. } => MidiEvent::ControlChange,
                            ChannelMessage::ProgramChange { .. } => MidiEvent::ProgramChange,
                            ChannelMessage::ChannelPressure { .. } => MidiEvent::ChannelPressure,
                            ChannelMessage::PitchBend { .. } => MidiEvent::PitchBend,
                        };
                        let status = Status::channel(event, *channel);
                        bytes.push(status.to_status_byte());

                        match message {
                            ChannelMessage::NoteOff { note, velocity } => {
                                bytes.push(note.value());
                                bytes.push(velocity.value());
                            }
                            ChannelMessage::NoteOn { note, velocity } => {
                                bytes.push(note.value());
                                bytes.push(velocity.value());
                            }
                            ChannelMessage::PolyphonicKeyPressure { note, pressure } => {
                                bytes.push(note.value());
                                bytes.push(pressure.value());
                            }
                            ChannelMessage::ControlChange { control, value } => {
                                bytes.push(control.value());
                                bytes.push(value.value());
                            }
                            ChannelMessage::ProgramChange { program } => {
                                bytes.push(program.value());
                            }
                            ChannelMessage::ChannelPressure { pressure } => {
                                bytes.push(pressure.value());
                            }
                            ChannelMessage::PitchBend { value } => {
                                bytes.push((value & 0x7F) as u8);
                                bytes.push((value >> 7) as u8);
                            }
                        }
                    }
                    MidiMessage::SystemCommon(event) => {
                        let status = Status::system_common(*event);
                        bytes.push(status.to_status_byte());
                    }
                    MidiMessage::RealTime(event) => {
                        let status = Status::real_time(*event);
                        bytes.push(status.to_status_byte());
                    }
                }
                bytes
            }
            EventType::Meta(event) => event.to_bytes(),
            EventType::Sysex(data) => data.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackEvent {
    pub v_time: Vql,
    pub event: EventType,
}

impl TrackEvent {
    pub fn new(v_time: Vql, event: EventType) -> Self {
        Self { v_time, event }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.v_time.encode_bytes();
        bytes.extend(self.event.to_bytes());
        bytes
    }

    pub fn track_name<S: AsRef<str>>(name: S) -> Self {
        let bytes = name.as_ref().as_bytes().to_vec();
        TrackEvent {
            v_time: Vql::zero(),
            event: EventType::Meta(crate::meta::MetaEvent::TrackName(bytes)),
        }
    }
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

    pub fn zero() -> Self {
        Self(0)
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
