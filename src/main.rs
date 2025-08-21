use frodo::channel::Channel;
use frodo::domain::{Note, Velocity};
use frodo::header::{Header, MidiFormat};
use frodo::message::{ChannelMessage, MidiMessage};
use frodo::meta::MetaEvent;
use frodo::track::{EventType, Track, TrackEvent, Vql};
use std::convert::TryFrom;
use std::fs;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 1. Definire la scala e i parametri
//     let channel = Channel::new(0)?;
//     let velocity = Velocity::new(100)?;
//     let note_duration = Vql::try_from(240)?; // Durata di una nota (in ticks)
//     let notes: Vec<u8> = vec![60, 62, 64, 65, 67, 69, 71, 72]; // Scala di Do Maggiore (C4-C5)

//     let mut track_events = Vec::new();

//     // Meta evento: Nome della traccia (buona pratica)
//     track_events.push(TrackEvent::new(
//         Vql::try_from(0)?,
//         EventType::Meta(MetaEvent::TrackName(b"C Major Scale".to_vec())),
//     ));

//     // 2. Creare gli eventi NoteOn/NoteOff
//     for (i, note_val) in notes.iter().enumerate() {
//         let note = Note::new(*note_val)?;
//         let delta_time = if i == 0 { Vql::try_from(0)? } else { note_duration };

//         // Note On
//         track_events.push(TrackEvent::new(
//             delta_time,
//             EventType::Midi(MidiMessage::Channel {
//                 channel,
//                 message: ChannelMessage::NoteOn { note, velocity },
//             }),
//         ));

//         // Note Off (dopo la durata della nota precedente)
//         track_events.push(TrackEvent::new(
//             note_duration,
//             EventType::Midi(MidiMessage::Channel {
//                 channel,
//                 message: ChannelMessage::NoteOff { note, velocity: Velocity::new(0)? },
//             }),
//         ));
//     }

//     // 3. Aggiungere l'evento di fine traccia (obbligatorio)
//     track_events.push(TrackEvent::new(
//         Vql::try_from(0)?,
//         EventType::Meta(MetaEvent::EndOfTrack),
//     ));

//     // 4. Creare la traccia e l'header
//     let track = Track::new(track_events);
//     let header = Header::new(MidiFormat::SingleTrack, 1, 480)?; // Formato 0, 1 traccia, 480 ticks per beat

//     // 5. Assemblare e scrivere il file
//     let mut midi_bytes = header.to_bytes();
//     midi_bytes.extend(track.to_bytes());

//     fs::write("scale.mid", midi_bytes)?;

//     println!("Successfully generated scale.mid");

//     Ok(())
// }

fn main() {
    let name = TrackEvent::track_name("La corsa di zio Michele");

    println!("{:?}", name);
}
