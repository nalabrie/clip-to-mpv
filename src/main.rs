use std::io::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use clipboard_win::{get_clipboard_string, set_clipboard_string};

fn main() -> Result<(), Error> {
    let mut result: String;
    let mut prev_result = String::new();
    let wait_duration = Duration::from_millis(250);
    set_clipboard_string("").expect("Clearing clipboard on first run");

    loop {
        result = get_clipboard_string().unwrap_or(prev_result.clone());

        if result == prev_result {
            sleep(wait_duration);
            continue;
        }

        prev_result = result.clone();
        println!("Now playing: {result}");
        Command::new("mpv.exe").arg("--mute").arg(result).spawn()?;
    }
}
