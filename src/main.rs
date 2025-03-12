/* This is AmirElfu */
// by reading this source code you will hate WayLand :D
// because it's not easy to get global keyboard input in WayLand
// but in X11 it's easy to get global keyboard input
use rdev::{listen, Event, EventType, Key};
use std::env;

mod playwav;

fn main() {
    //detect X11 or Wayland
    let display = env::var("WAYLAND_DISPLAY");
    println!("{:?}", display);
    match display {
        Ok(_) => wayland(),
        Err(_) => x11(),
    }
}

fn x11() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn wayland() {
    // SOON..
    // i dunno how to get global keyboard input in wayland :D
    println!("Hi, I know you want to run this fun program in Wayland,
    but I'm sorry it's not possible right now,
    because I don't know how to get global keyboard input in Wayland,
    but I'm sure it's possible, I just need to learn how to do it,
    so please run this program in X11, thank you :D");
    println!("Wayland Support is coming soon..");
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        let key_str = match key {
            // meow
            Key::KeyA => "01-0",
            Key::KeyB => "01-1",
            Key::KeyC => "02-0",
            Key::KeyD => "02-1",
            Key::KeyE => "03-0",
            Key::KeyF => "03-1",
            Key::KeyG => "04-0",
            Key::KeyH => "04-1",
            Key::KeyI => "05-0",
            Key::KeyJ => "05-1",
            Key::KeyK => "06-0",
            Key::KeyL => "06-1",
            Key::KeyM => "07-0",
            Key::KeyN => "07-1",
            Key::KeyO => "08-0",
            Key::KeyP => "08-1",
            Key::KeyQ => "09-0",
            Key::KeyR => "09-1",
            Key::KeyS => "0a-0",
            Key::KeyT => "0a-1",
            Key::KeyU => "0b-0",
            Key::KeyV => "0b-1",
            Key::KeyW => "0c-0",
            Key::KeyX => "0c-1",
            Key::KeyY => "0d-0",
            Key::KeyZ => "0d-1",
            Key::Num1 => "0e-0",
            Key::Num2 => "0e-1",
            Key::Num3 => "0f-0",
            Key::Num4 => "0f-1",
            Key::Num5 => "10-0",
            Key::Num6 => "10-1",
            Key::Num7 => "11-0",
            Key::Num8 => "11-1",
            Key::Num9 => "12-0",
            Key::Num0 => "12-1",
            Key::F1 => "13-0",
            Key::F2 => "13-1",
            Key::F3 => "14-0",
            Key::F4 => "14-1",
            Key::F5 => "15-0",
            Key::F6 => "15-1",
            Key::F7 => "16-0",
            Key::F8 => "16-1",
            Key::F9 => "17-0",
            Key::F10 => "17-1",
            Key::F11 => "18-0",
            Key::F12 => "18-1",
            Key::PrintScreen => "19-0",
            Key::ScrollLock => "19-1",
            Key::Pause => "1a-0",
            Key::Insert => "1a-1",
            Key::Home => "1b-0",
            Key::PageUp => "1b-1",
            Key::Delete => "1c-0",
            Key::End => "1c-1",
            Key::PageDown => "1d-0",
            Key::RightArrow => "1d-1",
            Key::LeftArrow => "1e-0",
            Key::DownArrow => "1e-1",
            Key::UpArrow => "1f-0",
            Key::NumLock => "1f-1",
            Key::Slash => "20-0",
            Key::BackSlash => "20-1",
            Key::Comma => "21-0",
            Key::Dot => "21-1",
            Key::SemiColon => "22-0",
            Key::Alt => "22-1",
            Key::ControlLeft => "23-0",
            Key::ControlRight => "23-1",
            Key::ShiftLeft => "24-0",
            Key::ShiftRight => "24-1",
            Key::Space => "25-0",
            Key::Tab => "25-1",
            Key::CapsLock => "26-0",
            Key::Function => "26-1",
            Key::MetaLeft => "27-0",
            Key::MetaRight => "27-1",
            Key::AltGr => "28-0",
            Key::Return => "28-1",
            Key::Escape => "29-0",
            Key::Backspace => "29-1",
            Key::Equal => "2a-0",
            Key::Minus => "2a-1",
            Key::LeftBracket => "2b-0",
            Key::RightBracket => "2b-1",
            Key::BackQuote => "2c-0",
            Key::Quote => "2c-1",
            _ => return,
        };

        if let Err(e) = playwav::play(key_str) {
            eprintln!("Error playing sound: {:?}", e);
        }
    }
}