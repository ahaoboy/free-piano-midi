use free_piano_midi::decode;

fn main() {
    let p = std::env::args()
        .nth(1)
        .expect("usage: free-piano-midi <FILE.mid>");
    let bytes = std::fs::read(&p).expect("read file error");
    let v = decode(bytes, None).expect("decode midi file error");
    let s = serde_json::to_string(&v).expect("serde json error");
    println!("{s}");
}
