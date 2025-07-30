use crate::error::MidiError;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Channel(u8);

impl TryFrom<u8> for Channel {
    type Error = MidiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0x0F {
            Ok(Channel(value))
        } else {
            Err(MidiError::InvalidChannel(value))
        }
    }
}

impl Channel {
    pub fn value(&self) -> u8 {
        self.0
    }
}
