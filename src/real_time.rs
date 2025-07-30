use crate::error::MidiError;
use std::convert::TryFrom;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealTimeMessage {
    TimingClock = 0xF8,
    Reserved1 = 0xF9,
    Start = 0xFA,
    Continue = 0xFB,
    Stop = 0xFC,
    Reserved2 = 0xFD,
    ActiveSensing = 0xFE,
    Reset = 0xFF,
}

impl TryFrom<u8> for RealTimeMessage {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xF8 => Ok(RealTimeMessage::TimingClock),
            0xF9 => Ok(RealTimeMessage::Reserved1),
            0xFA => Ok(RealTimeMessage::Start),
            0xFB => Ok(RealTimeMessage::Continue),
            0xFC => Ok(RealTimeMessage::Stop),
            0xFD => Ok(RealTimeMessage::Reserved2),
            0xFE => Ok(RealTimeMessage::ActiveSensing),
            0xFF => Ok(RealTimeMessage::Reset),
            _ => Err(MidiError::InvalidRealTimeMessage(value)),
        }
    }
}
