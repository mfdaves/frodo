use crate::channel::Channel;
use crate::domain::{Control, Note, Pressure, Program, Velocity};
use crate::error::MidiError;
use crate::real_time::RealTimeMessage;
use crate::system_common::SystemCommonEvent;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Channel { event_type: u8, channel: Channel },
    SystemCommon(SystemCommonEvent),
    RealTime(RealTimeMessage),
}

impl Status {
    pub fn channel(event_type: u8, channel: Channel) -> Self {
        Status::Channel {
            event_type,
            channel,
        }
    }

    pub fn system_common(sys_comm: SystemCommonEvent) -> Self {
        Status::SystemCommon(sys_comm)
    }

    pub fn real_time(real_time: RealTimeMessage) -> Self {
        Status::RealTime(real_time)
    }

    pub fn from_status_byte(byte: u8) -> Result<Self, MidiError> {
        if byte & 0x80 == 0 {
            return Err(MidiError::InvalidStatusByte);
        }

        match byte {
            0x80..=0xEF => {
                let event_type = (byte >> 4) & 0x07;
                let channel = Channel::try_from(byte & 0x0F)?;
                Ok(Status::Channel {
                    event_type,
                    channel,
                })
            }
            0xF0..=0xF7 => {
                let sys_val = byte & 0x0F;
                let system_common = SystemCommonEvent::try_from(sys_val)?;
                Ok(Status::SystemCommon(system_common))
            }
            0xF8..=0xFF => {
                let realtime = RealTimeMessage::try_from(byte)?;
                Ok(Status::RealTime(realtime))
            }
            _ => unreachable!(),
        }
    }

    pub fn to_status_byte(&self) -> u8 {
        match self {
            Status::Channel {
                event_type,
                channel,
            } => 0x80 | (event_type << 4) | channel.value(),
            Status::SystemCommon(sys) => 0xF0 | (*sys as u8),
            Status::RealTime(rt) => *rt as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelMessage {
    NoteOff { note: Note, velocity: Velocity },
    NoteOn { note: Note, velocity: Velocity },
    PolyphonicKeyPressure { note: Note, pressure: Pressure },
    ControlChange { control: Control, value: Control },
    ProgramChange { program: Program },
    ChannelPressure { pressure: Pressure },
    PitchBend { value: u16 },
}

impl ChannelMessage {
    pub fn event_type(&self) -> u8 {
        match self {
            ChannelMessage::NoteOff { .. } => 0,
            ChannelMessage::NoteOn { .. } => 1,
            ChannelMessage::PolyphonicKeyPressure { .. } => 2,
            ChannelMessage::ControlChange { .. } => 3,
            ChannelMessage::ProgramChange { .. } => 4,
            ChannelMessage::ChannelPressure { .. } => 5,
            ChannelMessage::PitchBend { .. } => 6,
        }
    }

    fn from_event_type(event_type: u8, data: &[u8]) -> Result<(ChannelMessage, usize), MidiError> {
        match event_type {
            0 => Ok((
                ChannelMessage::NoteOff {
                    note: Note::new(data[0])?,
                    velocity: Velocity::new(data[1])?,
                },
                2,
            )),
            1 => Ok((
                ChannelMessage::NoteOn {
                    note: Note::new(data[0])?,
                    velocity: Velocity::new(data[1])?,
                },
                2,
            )),
            2 => Ok((
                ChannelMessage::PolyphonicKeyPressure {
                    note: Note::new(data[0])?,
                    pressure: Pressure::new(data[1])?,
                },
                2,
            )),
            3 => Ok((
                ChannelMessage::ControlChange {
                    control: Control::new(data[0])?,
                    value: Control::new(data[1])?,
                },
                2,
            )),
            4 => Ok((
                ChannelMessage::ProgramChange {
                    program: Program::new(data[0])?,
                },
                1,
            )),
            5 => Ok((
                ChannelMessage::ChannelPressure {
                    pressure: Pressure::new(data[0])?,
                },
                1,
            )),
            6 => {
                let value = u16::from(data[1]) << 7 | u16::from(data[0]);
                Ok((ChannelMessage::PitchBend { value }, 2))
            }
            _ => Err(MidiError::InvalidEvent(event_type)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    Channel {
        channel: Channel,
        message: ChannelMessage,
    },
    SystemCommon(SystemCommonEvent),
    RealTime(RealTimeMessage),
}

impl MidiMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), MidiError> {
        let status = Status::from_status_byte(bytes[0])?;
        let (message, len) = match status {
            Status::Channel {
                event_type,
                channel,
            } => {
                let (msg, len) = ChannelMessage::from_event_type(event_type, &bytes[1..])?;
                (
                    MidiMessage::Channel {
                        channel,
                        message: msg,
                    },
                    len + 1,
                )
            }
            Status::SystemCommon(evt) => (MidiMessage::SystemCommon(evt), 1),
            Status::RealTime(rt) => (MidiMessage::RealTime(rt), 1),
        };
        Ok((message, len))
    }
}
