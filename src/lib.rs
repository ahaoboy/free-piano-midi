use midly::{num::u7, MidiMessage, Smf, Timing, TrackEventKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteEvent {
    pub code: u8,
    pub start: u32,
    pub end: u32,
}

#[wasm_bindgen]
pub fn decode(bytes: Vec<u8>, bpm: Option<u32>) -> Option<Vec<NoteEvent>> {
    let smf = Smf::parse(&bytes).expect("Failed to parse MIDI file");

    let ticks_per_beat = match smf.header.timing {
        Timing::Metrical(ticks) => ticks.as_int() as u32,
        _ => panic!("Only metrical timing is supported"),
    };

    // Default tempo (120 BPM = 500,000 microseconds per beat)
    let bpm = bpm.unwrap_or(120);
    let mut microseconds_per_beat = 60_000_000 / bpm;
    let mut notes: Vec<NoteEvent> = Vec::new();

    for track in smf.tracks {
        let mut current_ticks = 0; // Absolute time in ticks
        let mut active_notes: HashMap<(u8, u7), u32> = HashMap::new(); // (note, key) -> start time

        for event in track {
            current_ticks += event.delta.as_int();

            match event.kind {
                TrackEventKind::Midi {
                    channel: _,
                    message,
                } => match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        let start_time =
                            current_ticks * microseconds_per_beat / (ticks_per_beat * 1_000_000);
                        active_notes.insert((key.as_int(), key), start_time);
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        if let Some(start) = active_notes.remove(&(key.as_int(), key)) {
                            let end = current_ticks * microseconds_per_beat
                                / (ticks_per_beat * 1_000_000);
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
                    microseconds_per_beat = tempo.as_int();
                }
                _ => {}
            }
        }
    }

    Some(notes)
}
