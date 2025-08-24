use frodo::meta::MetaEvent;
use frodo::*;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;

// --- MIDI HELPER FUNCTIONS ---

struct TimedEvent { time: u32, event: EventType }
fn add_note(events: &mut Vec<TimedEvent>, time: u32, ch: u8, note: u8, vel: u8, dur: u32) {
    let channel = Channel::new(ch).unwrap();
    let note_val = Note::new(note.min(127)).unwrap();
    let vel_on = Velocity::new(vel.min(127)).unwrap();
    events.push(TimedEvent { time, event: EventType::Midi(MidiMessage::Channel { channel, message: ChannelMessage::NoteOn { note: note_val, velocity: vel_on } }) });
    events.push(TimedEvent { time: time + dur, event: EventType::Midi(MidiMessage::Channel { channel, message: ChannelMessage::NoteOff { note: note_val, velocity: Velocity::new(0).unwrap() } }) });
}

// --- MAIN COMPOSITION ---

fn main() {
    let division = 480;
    let header = Header::new(MidiFormat::SingleTrack, 1, division).unwrap();
    let mut timed_events: Vec<TimedEvent> = Vec::new();

    timed_events.push(TimedEvent { time: 0, event: EventType::Meta(MetaEvent::TrackName("Notte Fonda".as_bytes().to_vec())) });
    timed_events.push(TimedEvent { time: 0, event: EventType::Meta(MetaEvent::SetTempo(60_000_000 / 85)) }); // 85 BPM

    // --- Instruments ---
    timed_events.push(TimedEvent { time: 0, event: EventType::Midi(MidiMessage::Channel { channel: Channel::new(0).unwrap(), message: ChannelMessage::ProgramChange { program: Program::new(32).unwrap() } }) }); // Acoustic Bass
    timed_events.push(TimedEvent { time: 0, event: EventType::Midi(MidiMessage::Channel { channel: Channel::new(1).unwrap(), message: ChannelMessage::ProgramChange { program: Program::new(4).unwrap() } }) }); // Electric Piano 1 (Rhodes)
    timed_events.push(TimedEvent { time: 0, event: EventType::Midi(MidiMessage::Channel { channel: Channel::new(2).unwrap(), message: ChannelMessage::ProgramChange { program: Program::new(66).unwrap() } }) }); // Tenor Sax
    timed_events.push(TimedEvent { time: 0, event: EventType::Midi(MidiMessage::Channel { channel: Channel::new(3).unwrap(), message: ChannelMessage::ProgramChange { program: Program::new(48).unwrap() } }) }); // String Ensemble 1

    let ticks_per_beat = division as u32;
    let ticks_per_bar = ticks_per_beat * 4;
    let ticks_per_16th = ticks_per_beat / 4;
    let swing_offset = ticks_per_16th / 3; // Classic swing timing

    // Jazzy Chord Progression: Am7 - Dm7 - G7 - Cmaj7
    let chord_progression = [
        vec![57, 60, 64, 67], // Am7
        vec![50, 53, 57, 62], // Dm7
        vec![55, 59, 62, 65], // G7
        vec![48, 52, 55, 59], // Cmaj7
    ];
    let bass_progression = [45, 50, 43, 48]; // A, D, G, C

    // --- SONG STRUCTURE (72 bars, ~3.5 mins at 85 BPM) ---
    for bar in 0..72 {
        let bar_start_time = bar as u32 * ticks_per_bar;
        let chord_index = (bar / 2) % 4;
        let current_chord = &chord_progression[chord_index];
        let current_bass = bass_progression[chord_index];

        // --- INTRO (Bars 0-8) ---
        if bar < 8 {
            // Rhodes plays the main loop
            if bar % 2 == 0 {
                for &note in current_chord {
                    add_note(&mut timed_events, bar_start_time, 1, note + 12, 65, ticks_per_bar);
                }
            }
        }

        // --- VERSE/CHORUS SECTIONS (Bars 8-64) ---
        if bar >= 8 && bar < 64 {
            // Boom-bap Drums
            // Kick
            add_note(&mut timed_events, bar_start_time, 9, 36, 120, ticks_per_16th);
            add_note(&mut timed_events, bar_start_time + ticks_per_16th * 3, 9, 36, 100, ticks_per_16th);
            add_note(&mut timed_events, bar_start_time + ticks_per_beat * 2, 9, 36, 120, ticks_per_16th);
            // Snare
            add_note(&mut timed_events, bar_start_time + ticks_per_beat, 9, 38, 127, ticks_per_16th);
            add_note(&mut timed_events, bar_start_time + ticks_per_beat * 3, 9, 38, 127, ticks_per_16th);
            // Hi-hats with swing
            for step in 0..8 {
                let time = bar_start_time + step * (ticks_per_beat / 2);
                let offset = if step % 2 == 1 { swing_offset } else { 0 };
                let vel = if step % 2 == 0 { 90 } else { 70 };
                add_note(&mut timed_events, time + offset, 9, 42, vel, ticks_per_16th);
            }

            // Bassline
            add_note(&mut timed_events, bar_start_time, 0, current_bass, 110, ticks_per_beat);
            add_note(&mut timed_events, bar_start_time + ticks_per_beat * 2, 0, current_bass, 100, ticks_per_beat);
            add_note(&mut timed_events, bar_start_time + ticks_per_beat * 3 + ticks_per_beat / 2, 0, current_bass + 7, 80, ticks_per_beat / 2); // Octave fill

            // Rhodes
            if bar % 2 == 0 {
                for &note in current_chord {
                    add_note(&mut timed_events, bar_start_time, 1, note + 12, 75, ticks_per_beat * 2);
                }
            }
            
            // Sax Lick (Chorus)
            if (bar >= 24 && bar < 32) || (bar >= 48 && bar < 56) {
                let sax_lick = [
                    (current_chord[3] + 12, 0, ticks_per_beat),
                    (current_chord[2] + 12, ticks_per_beat, ticks_per_beat / 2),
                    (current_chord[1] + 12, ticks_per_beat + ticks_per_beat / 2, ticks_per_beat * 2),
                ];
                for &(note, offset, dur) in &sax_lick {
                    add_note(&mut timed_events, bar_start_time + offset, 2, note, 80, dur);
                }
            }
            
            // String Pad
            if bar >= 24 {
                 add_note(&mut timed_events, bar_start_time, 3, current_chord[0], 40, ticks_per_bar);
            }
        }
        
        // --- OUTRO (Bars 64-72) ---
        if bar >= 64 {
            // Rhodes fades out
            if bar % 2 == 0 {
                for &note in current_chord {
                    add_note(&mut timed_events, bar_start_time, 1, note + 12, 60 - ((bar - 64) * 5) as u8, ticks_per_bar);
                }
            }
            // Bass plays root notes
            add_note(&mut timed_events, bar_start_time, 0, current_bass, 90, ticks_per_bar);
        }
    }

    // --- Finalization ---
    timed_events.sort_by_key(|e| e.time);
    let mut track_events: Vec<TrackEvent> = Vec::new();
    let mut last_time = 0;
    for timed_event in timed_events {
        let delta_time = timed_event.time - last_time;
        track_events.push(TrackEvent::new(Vql::try_from(delta_time).unwrap(), timed_event.event));
        last_time = timed_event.time;
    }
    track_events.push(TrackEvent::end_track());

    let track = Track::new(track_events);
    let midi = Midi::new(header, vec![track]);
    let bytes = midi.to_bytes();

    let mut file = File::create("notte_fonda.mid").unwrap();
    file.write_all(&bytes).unwrap();

    println!("File 'notte_fonda.mid' created. Perfect for a late night drive.");
}