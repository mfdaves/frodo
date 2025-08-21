use crate::error::MidiError;

macro_rules! midi_value {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(u8);

        impl TryFrom<u8> for $name {
            type Error = MidiError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                if value <= 127 {
                    Ok($name(value))
                } else {
                    Err(MidiError::InvalidMidiValue {
                        kind: $kind,
                        value,
                    })
                }
            }
        }

        impl $name {
            pub fn new(value: u8) -> Result<Self, MidiError> {
                Self::try_from(value)
            }

            pub fn value(&self) -> u8 {
                self.0
            }
        }
    };
}

midi_value!(Note, "Note");
midi_value!(Velocity, "Velocity");
midi_value!(Pressure, "Pressure");
midi_value!(Program, "Program");
midi_value!(Control, "Control");
