use midly::{MidiMessage, Smf, Timing, TrackEventKind};
use serde::{Deserialize, Serialize};
// use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
// Structure to hold note events for JSON output
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NoteEvent {
    pub code: u8,
    pub start: f64, // in seconds
    pub end: f64,   // in seconds
}
// fn midi_to_note_custom(midi_note: u8) -> String {
//     // 音符名称数组，索引对应 MIDI 编号 % 12
//     let note_names = [
//         "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
//     ];

//     // 检查 MIDI 编号是否在有效范围内 (0-127)
//     if midi_note > 127 {
//         return String::from("Invalid MIDI note: must be between 0 and 127");
//     }

//     // 计算音符索引和八度
//     let note_index = (midi_note % 12) as usize;
//     let octave = (midi_note / 12) as i32 - 1;

//     // 拼接音符名称和八度
//     format!("{}{}", note_names[note_index], octave)
// }

// fn midi_to_note_custom(note: u8) -> String {
//     // 参考点：MIDI=69 对应 F#5
//     const REF_MIDI: i32 = 69;
//     const REF_NAME: &str = "F#5";

//     // 所有半音的名称
//     let names = [
//         "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
//     ];

//     // 解析参考音符名和八度
//     let (ref_base, ref_oct) = {
//         let s = REF_NAME;
//         // 分离音名（可能含 #）和数字部分
//         let split = s.find(|c: char| c.is_ascii_digit()).unwrap();
//         let base = &s[..split];
//         let oct: i32 = s[split..].parse().expect("八度解析失败");
//         (base, oct)
//     };
//     // 得到参考音名在 names 中的索引
//     let ref_index = names
//         .iter()
//         .position(|&n| n == ref_base)
//         .expect("参考音名无效") as i32;

//     // 计算当前 note 相对于参考音符的半音差（可以为负）
//     let delta = note as i32 - REF_MIDI;
//     // 总的半音偏移 = 参考索引 + delta
//     let total = ref_index + delta;
//     // 规范化到 0–11
//     let idx = ((total % 12) + 12) % 12;
//     // 八度偏移：向下整除 12
//     let oct_offset = (total as f64 / 12.0).floor() as i32;
//     let octave = ref_oct + oct_offset;

//     format!("{}{}", names[idx as usize], octave)
// }

// // Function to create a Virtual Piano key mapping (simplified example)
// fn create_key_map() -> HashMap<u8, char> {
//     let mut key_map = HashMap::new();
//     // Example mapping based on Virtual Piano's common layout
//     key_map.insert(60, 'a'); // C4
//     key_map.insert(62, 's'); // D4
//     key_map.insert(64, 'd'); // E4
//     key_map.insert(65, 'f'); // F4
//     key_map.insert(67, 'g'); // G4
//     key_map.insert(69, 'h'); // A4
//     key_map.insert(71, 'j'); // B4
//     key_map.insert(72, 'k'); // C5
//                              // Add more mappings as needed (refer to Virtual Piano's key layout)
//     key_map
// }

#[wasm_bindgen]
pub fn decode(bytes: Vec<u8>) -> Option<Vec<NoteEvent>> {
    // Read the MIDI file
    // let bytes = fs::read("b.mid").expect("Failed to read MIDI file");
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
                TrackEventKind::Meta(msg) => {
                    if let midly::MetaMessage::Tempo(tempo) = msg {
                        microseconds_per_beat = tempo.as_int() as f64;
                    }
                }
                _ => {}
            }
        }
    }

    // Generate JSON output
    // let json_output = json!(notes);
    // println!("{}", json_output.to_string());
    // Some(json_output.to_string())
    Some(notes)
}
