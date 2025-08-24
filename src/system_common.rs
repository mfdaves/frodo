use crate::error::MidiError;
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemCommonEvent {
    SysExStart = 0,
    MTCQuarterFrame = 1,
    SongPositionPointer = 2,
    SongSelect = 3,
    Undefined1 = 4,
    Undefined2 = 5,
    TuneRequest = 6,
    EndOfSysEx = 7,
}

impl TryFrom<u8> for SystemCommonEvent {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SystemCommonEvent::SysExStart),
            0x1 => Ok(SystemCommonEvent::MTCQuarterFrame),
            0x2 => Ok(SystemCommonEvent::SongPositionPointer),
            0x3 => Ok(SystemCommonEvent::SongSelect),
            0x4 => Ok(SystemCommonEvent::Undefined1),
            0x5 => Ok(SystemCommonEvent::Undefined2),
            0x6 => Ok(SystemCommonEvent::TuneRequest),
            0x7 => Ok(SystemCommonEvent::EndOfSysEx),
            _ => Err(MidiError::InvalidSystemCommonEvent(value)),
        }
    }
}
