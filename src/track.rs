use crate::chunktype::ChunkType;
use crate::domain::*;
use crate::error::MidiError;
use crate::message::{ChannelMessage, MidiMessage, Status};
use crate::{Channel, Velocity};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Track {
    chunk_type: ChunkType,
    length: u32,
    events: Vec<TrackEvent>,
    name: String,
}

impl Track {
    pub fn new(events: Vec<TrackEvent>) -> Self {
        let len = events.iter().map(|x| x.to_bytes().len() as u32).sum();
        Self {
            chunk_type: ChunkType::Track,
            length: len, // Will be calculated in to_bytes
            events,
            name: Default::default(),
        }
    }

    // maybe a add_name_event method
    // for add in the first place

    pub fn with_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.name = name.as_ref().to_string();
        self.events.insert(0, TrackEvent::track_name(name));
        self
    }

    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        self.name = name.as_ref().to_string();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // use std::collections::HashMap;

        let mut cache: HashMap<&TrackEvent, Vec<u8>> = HashMap::new();

        let total_len: usize = self
            .events
            .iter()
            .map(|event| cache.entry(event).or_insert_with(|| event.to_bytes()).len())
            .sum();

        let mut bytes = Vec::with_capacity(4 + 4 + total_len); // chunk_type + length + track_data

        // Chunk type
        bytes.extend_from_slice(&self.chunk_type.as_bytes());

        bytes.extend_from_slice(&(total_len as u32).to_be_bytes());

        self.events.iter().for_each(|event| {
            let event_bytes = cache.get(event).unwrap();
            bytes.extend_from_slice(event_bytes);
        });

        bytes
    }

    pub fn add_event(&mut self, event: TrackEvent) {
        // if events is empty we add first track name and then we add
        self.events.push(event);
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }
}

// need to put track_name first and EOT
// if a TRACKANME or EOT will found I
// impl From<&[TrackEvent]> for Track {
//     fn from(value: &[TrackEvent]) -> Self {
//         let events = value.to_vec();
//     }
// }

impl Default for Track {
    fn default() -> Self {
        Self {
            chunk_type: ChunkType::Track,
            length: 0,
            events: Vec::new(),
            name: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                        let event_type = message.event_type();
                        let status = Status::channel(event_type, *channel);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn meta_event(event: crate::meta::MetaEvent) -> Self {
        Self {
            v_time: Vql::zero(),
            event: EventType::Meta(event),
        }
    }
    pub fn track_name<S: AsRef<str>>(name: S) -> Self {
        Self::meta_event(crate::meta::MetaEvent::TrackName(
            name.as_ref().as_bytes().to_vec(),
        ))
    }
    pub fn end_track() -> Self {
        Self::meta_event(crate::meta::MetaEvent::EndOfTrack)
    }

    pub fn note_on(delta_time: Vql, channel: Channel, note: Note, velocity: Velocity) -> Self {
        let message = ChannelMessage::NoteOn { note, velocity };
        Self {
            v_time: delta_time,
            event: EventType::Midi(MidiMessage::Channel { channel, message }),
        }
    }

    pub fn note_off(delta_time: Vql, channel: Channel, note: Note, velocity: Velocity) -> Self {
        let message = ChannelMessage::NoteOff { note, velocity };
        Self {
            v_time: delta_time,
            event: EventType::Midi(MidiMessage::Channel { channel, message }),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
