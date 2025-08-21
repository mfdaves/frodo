use crate::channel::Channel;
use crate::domain::{Control, Note, Pressure, Program, Velocity};
use crate::error::MidiError;
use crate::event::MidiEvent;
use crate::real_time::RealTimeMessage;
use crate::system_common::SystemCommonEvent;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Channel { event: MidiEvent, channel: Channel },
    SystemCommon(SystemCommonEvent),
    RealTime(RealTimeMessage),
}

impl Status {
    pub fn channel(event: MidiEvent, channel: Channel) -> Self {
        Status::Channel { event, channel }
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
                let event = MidiEvent::try_from((byte >> 4) & 0x07)?;
                let channel = Channel::try_from(byte & 0x0F)?;
                Ok(Status::Channel { event, channel })
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
            Status::Channel { event, channel } => 0x80 | ((*event as u8) << 4) | channel.value(),
            Status::SystemCommon(sys) => 0xF0 | (*sys as u8),
            Status::RealTime(rt) => *rt as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelMessage {
    NoteOff { note: Note, velocity: Velocity },
    NoteOn { note: Note, velocity: Velocity },
    PolyphonicKeyPressure { note: Note, pressure: Pressure },
    ControlChange { control: Control, value: Control },
    ProgramChange { program: Program },
    ChannelPressure { pressure: Pressure },
    PitchBend { value: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Status::Channel { event, channel } => {
                let (msg, len) = Self::parse_channel_message(event, &bytes[1..])?;
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

    fn parse_channel_message(
        event: MidiEvent,
        data: &[u8],
    ) -> Result<(ChannelMessage, usize), MidiError> {
        match event {
            MidiEvent::NoteOff => Ok((
                ChannelMessage::NoteOff {
                    note: Note::new(data[0])?,
                    velocity: Velocity::new(data[1])?,
                },
                2,
            )),
            MidiEvent::NoteOn => Ok((
                ChannelMessage::NoteOn {
                    note: Note::new(data[0])?,
                    velocity: Velocity::new(data[1])?,
                },
                2,
            )),
            MidiEvent::PolyphonicKeyPressure => Ok((
                ChannelMessage::PolyphonicKeyPressure {
                    note: Note::new(data[0])?,
                    pressure: Pressure::new(data[1])?,
                },
                2,
            )),
            MidiEvent::ControlChange => Ok((
                ChannelMessage::ControlChange {
                    control: Control::new(data[0])?,
                    value: Control::new(data[1])?,
                },
                2,
            )),
            MidiEvent::ProgramChange => Ok((
                ChannelMessage::ProgramChange {
                    program: Program::new(data[0])?,
                },
                1,
            )),
            MidiEvent::ChannelPressure => Ok((
                ChannelMessage::ChannelPressure {
                    pressure: Pressure::new(data[0])?,
                },
                1,
            )),
            MidiEvent::PitchBend => {
                let value = u16::from(data[1]) << 7 | u16::from(data[0]);
                Ok((ChannelMessage::PitchBend { value }, 2))
            }
        }
    }
}
