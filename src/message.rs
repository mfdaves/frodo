use crate::channel::Channel;
use crate::error::MidiError;
use crate::event::MidiEvent;
use crate::real_time::RealTimeMessage;
use crate::system_common::SystemCommonEvent;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiMessage {
    Channel { event: MidiEvent, channel: Channel },
    SystemCommon(SystemCommonEvent),
    RealTime(RealTimeMessage),
}

impl MidiMessage {
    pub fn from_status_byte(byte: u8) -> Result<Self, MidiError> {
        if byte & 0x80 == 0 {
            return Err(MidiError::InvalidStatusByte);
        }

        match byte {
            0x80..=0xEF => {
                let event = MidiEvent::try_from((byte >> 4) & 0x07)?;
                let channel = Channel::try_from(byte & 0x0F)?;
                Ok(MidiMessage::Channel { event, channel })
            }
            0xF0..=0xF7 => {
                let sys_val = byte & 0x0F;
                let system_common = SystemCommonEvent::try_from(sys_val)?;
                Ok(MidiMessage::SystemCommon(system_common))
            }
            0xF8..=0xFF => {
                let realtime = RealTimeMessage::try_from(byte)?;
                Ok(MidiMessage::RealTime(realtime))
            }
            _ => unreachable!(),
        }
    }

    pub fn to_status_byte(&self) -> u8 {
        match self {
            MidiMessage::Channel { event, channel } => {
                0x80 | ((*event as u8) << 4) | channel.value()
            }
            MidiMessage::SystemCommon(sys) => 0xF0 | (*sys as u8),
            MidiMessage::RealTime(rt) => *rt as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiData {
    pub data1: u8,
    pub data2: Option<u8>,
}

impl MidiData {
    pub fn new(data1: u8, data2: Option<u8>) -> Self {
        Self { data1, data2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompleteMidiMessage {
    pub status: MidiMessage,
    pub data: MidiData,
}

impl CompleteMidiMessage {
    pub fn from_bytes(status_byte: u8, data1: u8, data2: Option<u8>) -> Result<Self, MidiError> {
        let status = MidiMessage::from_status_byte(status_byte)?;
        Ok(Self {
            status,
            data: MidiData::new(data1, data2),
        })
    }
}
