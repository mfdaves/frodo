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
    pub fn new(value: u8) -> Result<Self, MidiError> {
        Self::try_from(value)
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_channel() {
        let value_err: u8 = 255;
        let value_ok: u8 = 0;
        assert_eq!(
            Channel::new(value_err).unwrap_err(),
            MidiError::InvalidChannel(value_err)
        );
        assert_eq!(Channel::new(value_ok).unwrap().value(), value_ok)
    }
}
