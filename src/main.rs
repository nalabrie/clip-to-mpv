use std::io::Error;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use arboard::Clipboard;
use ctrlc;
use url::Url;

// === GLOBAL CONSTANTS ===

// how long to wait between clipboard reads
const WAIT_DURATION: Duration = Duration::from_millis(250);

// mpv command & args
// TODO: check if mpv is installed before running
// TODO: check if yt-dlp is installed so mpv can play remote URLs
// TODO: check if ffmpeg is installed in case a video needs to be combined with audio on the fly by yt-dlp
const MPV_COMMAND: &str = if cfg!(windows) { "mpv.exe" } else { "mpv" };
const MPV_ARG_MUTE: &str = "--mute";

// === FUNCTIONS ===

/// Checks if a String is a valid URL. Returns `true` when valid.
///
/// # Arguments
///
/// * `url`: String reference of URL to be checked for a valid format.
///
/// returns: bool
///
/// # Examples
///
/// ```
///  if validate_url(my_url_string) {
///      /* code to execute when valid URL is found */
///  }
/// ```
fn validate_url(url: &String) -> bool {
    Url::parse(url).is_ok()
}

// === MAIN ===

fn main() -> Result<(), Error> {
    // print app welcome message
    let version = env!("CARGO_PKG_VERSION");
    println!("clip-to-mpv version {version}");
    println!("Press Ctrl+C to exit\n");

    // init clipboard
    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Error initializing clipboard"),
    ));
    // init clipboard clone for Ctrl+C handler
    let clipboard_ctrlc_handler_clone = Arc::clone(&clipboard);

    // set up Ctrl+C handler
    // TODO: handle window close event, not just Ctrl+C
    ctrlc::set_handler(move || {
        println!("\nClearing clipboard and exiting...");
        let mut clipboard = clipboard_ctrlc_handler_clone.lock().unwrap();
        clipboard
            .clear()
            .expect("Error clearing clipboard before closing");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // initialize variables for main loop
    let mut result: String;
    let mut prev_result = String::new();

    // clear clipboard on first run
    {
        print!("Clearing clipboard before starting... ");
        let mut clipboard = clipboard.lock().unwrap();
        clipboard
            .clear()
            .expect("Error clearing clipboard on first run");
        println!("done\n");
    }

    // main loop
    loop {
        {
            let mut clipboard = clipboard.lock().unwrap();
            result = clipboard.get_text().unwrap_or_default();
        }

        if result.is_empty() || result == prev_result || !validate_url(&result) {
            // wait before next clipboard read to avoid high CPU usage
            sleep(WAIT_DURATION);
            continue;
        }

        prev_result = result.clone();
        println!("Now playing: {result}");
        Command::new(MPV_COMMAND)
            .arg(MPV_ARG_MUTE)
            .arg(result)
            .spawn()
            .expect("Error launching mpv with URL argument from clipboard");
    }
}
// sample URL: https://youtu.be/9FLRHejWAo8
