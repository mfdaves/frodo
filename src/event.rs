use crate::error::MidiError;
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiEvent {
    NoteOff = 0,
    NoteOn = 1,
    PolyphonicKeyPressure = 2,
    ControlChange = 3,
    ProgramChange = 4,
    ChannelPressure = 5,
    PitchBend = 6,
}

impl TryFrom<u8> for MidiEvent {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MidiEvent::NoteOff),
            1 => Ok(MidiEvent::NoteOn),
            2 => Ok(MidiEvent::PolyphonicKeyPressure),
            3 => Ok(MidiEvent::ControlChange),
            4 => Ok(MidiEvent::ProgramChange),
            5 => Ok(MidiEvent::ChannelPressure),
            6 => Ok(MidiEvent::PitchBend),
            _ => Err(MidiError::InvalidEvent(value)),
        }
    }
}


