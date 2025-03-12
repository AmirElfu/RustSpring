use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub fn play(key: &str) -> Result<(), Box<dyn Error>> {
    let key = key.to_string();
    thread::spawn(move || {
        if let Err(e) = play_sound(&key) {
            eprintln!("Error playing sound: {:?}", e);
        }
    });
    Ok(())
}

fn play_sound(key: &str) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file_path = format!("wav/{}.wav", key);
    let file = File::open(file_path)?;
    let source = Decoder::new(BufReader::new(file))?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}