extern crate cabot;
extern crate pancurses;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use pancurses as pc;

mod wave_gen;
mod spotify;

struct Screen;

impl Drop for Screen {
    fn drop(&mut self) {
        pc::endwin();
    }
}


fn get_bearer() -> Option<String> {
    std::env::var("SPOTIFY_OAUTH").ok()
}

#[cfg(target_os = "macos")]
fn sync_song() {
    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Spotify\" to play")
        .spawn();
}
//No op
#[cfg(not(target_os = "macos"))]
fn sync_song() {}

fn main() {
    let id = match std::env::args().skip(1).next() {
        Some(id) => id,
        None => {
            eprintln!("Must pass song id as argument");
            std::process::exit(1);
        }
    };

    let _screen_context = Screen;
    let window = pc::initscr();
    pc::curs_set(0);
    let (lines, rows) = window.get_max_yx();

    let segments = get_bearer()
        .map(|bearer| spotify::get_audio_segments(&id, &bearer))
        .unwrap_or_else(|| {
            pc::endwin();
            eprintln!("Environment variable `SPOTIFY_OAUTH` not set");
            std::process::exit(1);
        });

    if segments.len() == 0 {
        pc::endwin();
        eprintln!("No data, check if `SPOTIFY_OAUTH` is expired");
        std::process::exit(1);
    }

    for i in 1..4 {
        println!("{}", i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    sync_song();

    let yoffset = (rows / 2) - segments[0].pitches.len() as i32;

    for segment in &segments {
        window.clear();
        window.mvprintw(0, 0, &segment.start.to_string());

        for (i, pitch) in segment.pitches.iter().enumerate() {
            let row = wave_gen::gen_row((*pitch * 10f32) as u16);
            wave_gen::draw_row(&window, row, (i + 1) as i32 + yoffset, lines);
        }
        std::thread::sleep(std::time::Duration::from_millis(
            (segment.duration * 1000f64) as u64,
        ));
    }
}
