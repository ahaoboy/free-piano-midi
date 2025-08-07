use midly::{MidiMessage, Smf, Timing, TrackEventKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteEvent {
    pub code: u8,
    pub start: f64,
    pub end: f64,
}

#[wasm_bindgen]
pub fn decode(bytes: Vec<u8>) -> Option<Vec<NoteEvent>> {
    let smf = Smf::parse(&bytes).expect("Failed to parse MIDI file");

    // Get timing information (ticks per beat)
    let ticks_per_beat = match smf.header.timing {
        Timing::Metrical(ticks) => ticks.as_int(),
        _ => panic!("Only metrical timing is supported"),
    };

    // Default tempo (120 BPM = 500,000 microseconds per beat)
    let mut microseconds_per_beat = 500_000.0;
    let mut notes: Vec<NoteEvent> = Vec::new();
    // let key_map = create_key_map();

    // Process each track
    for track in smf.tracks {
        let mut current_ticks = 0.0; // Absolute time in ticks
        let mut active_notes: HashMap<(u8,), f64> = HashMap::new(); // (note, key) -> start time

        for event in track {
            current_ticks += event.delta.as_int() as f64;

            match event.kind {
                TrackEventKind::Midi {
                    channel: _,
                    message,
                } => match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        // let keyboard_key = midi_to_note_custom(key.as_int());
                        let start_time = current_ticks * microseconds_per_beat
                            / (ticks_per_beat as f64 * 1_000_000.0);
                        active_notes.insert((key.as_int(),), start_time);
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        // let keyboard_key = midi_to_note_custom(key.as_int());
                        if let Some(start) = active_notes.remove(&(key.as_int(),)) {
                            let end = current_ticks * microseconds_per_beat
                                / (ticks_per_beat as f64 * 1_000_000.0);
                            notes.push(NoteEvent {
                                start,
                                code: key.as_int(),
                                end,
                            });
                        }
                    }
                    _ => {}
                },
                TrackEventKind::Meta(midly::MetaMessage::Tempo(tempo)) => {
                    microseconds_per_beat = tempo.as_int() as f64;
                }
                _ => {}
            }
        }
    }

    Some(notes)
}
