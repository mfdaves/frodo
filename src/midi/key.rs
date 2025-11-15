use crate::domain::Note;
use std::convert::TryFrom;
use crate::error::MidiError;

impl TryFrom<&str> for Note {
    type Error = MidiError;

    fn try_from(value: &str) -> Result<Note, Self::Error> {
        if value.len() < 2 {
            return Err(MidiError::InvalidNote);
        }

        let octave_char = value.chars().last().unwrap();
        let octave = octave_char.to_digit(10).ok_or(MidiError::InvalidNote)? as i8;
        let note_str = &value[0..value.len() - 1];

        let note_index = match note_str {
            "C"  => 0,
            "C#" | "Db" => 1,
            "D"  => 2,
            "D#" | "Eb" => 3,
            "E"  => 4,
            "F"  => 5,
            "F#" | "Gb" => 6,
            "G"  => 7,
            "G#" | "Ab" => 8,
            "A"  => 9,
            "A#" | "Bb" => 10,
            "B"  => 11,
            _ => return Err(MidiError::InvalidNote),
        };

        let midi_number = ((octave + 1) * 12 + note_index) as u8;
        midi_number.try_into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_notes() {
        let c4 = Note::try_from("C4").unwrap();
        assert_eq!(c4.value(), 60);

        let dsharp3 = Note::try_from("D#3").unwrap();
        assert_eq!(dsharp3.value(), 51);

        let fsharp2 = Note::try_from("F#2").unwrap();
        assert_eq!(fsharp2.value(), 42);

        let bb1 = Note::try_from("Bb1").unwrap();
        assert_eq!(bb1.value(), 34);

        let a0 = Note::try_from("A0").unwrap();
        assert_eq!(a0.value(), 21);
    }

    #[test]
    fn test_invalid_notes() {
        assert!(Note::try_from("H4").is_err());
        assert!(Note::try_from("C").is_err());
        assert!(Note::try_from("").is_err());
        assert!(Note::try_from("Cb4").is_err());
    }
}
