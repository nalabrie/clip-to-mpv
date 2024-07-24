use std::io::Error;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use arboard::Clipboard;
use ctrlc;
use url::Url;
use which::which;

// TODO: make tests
// TODO: proper logging, not just println
// TODO: maybe add a config file and/or command line arguments?

// === GLOBAL CONSTANTS ===

// how long to wait between clipboard reads
const WAIT_DURATION: Duration = Duration::from_millis(250);

// mpv command & default args
const MPV_COMMAND: &str = if cfg!(windows) { "mpv.exe" } else { "mpv" };
const MPV_ARG_MUTE: &str = "--mute";

// other external app commands
const YT_DLP_COMMAND: &str = "yt-dlp";
const FFMPEG_COMMAND: &str = "ffmpeg";

// === FUNCTIONS ===

/// Checks if a String is a valid URL. Returns `true` when valid, `false` otherwise.
/// # Arguments
/// * `url`: String reference of URL to be checked for a valid format.
/// # Examples
/// ```
///  if validate_url(my_url_string) {
///      /* code to execute when valid URL is found */
///  }
/// ```
fn validate_url(url: &String) -> bool {
    Url::parse(url).is_ok()
}

/// Checks for the presence of external apps in the system PATH. Returns `true` when all **required** apps are found.
/// Prints error messages for missing **required** apps. Prints a warning message for missing **optional** apps.
///
/// Returns `true` when all **required** apps are found, `false` otherwise.
/// # Examples
/// ```
/// if check_for_external_apps() {
///    /* code to execute when all apps are found */
/// }
/// ```
fn check_for_external_apps() -> bool {
    let mpv_exists = which(MPV_COMMAND).is_ok();
    let yt_dlp_exists = which(YT_DLP_COMMAND).is_ok();
    let ffmpeg_exists = which(FFMPEG_COMMAND).is_ok();

    if !mpv_exists {
        eprintln!("[Error]: mpv not found in PATH");
    }
    if !yt_dlp_exists {
        eprintln!("[Error]: yt-dlp not found in PATH");
    }
    if !ffmpeg_exists {
        println!("[Warning]: ffmpeg not found in PATH. ffmpeg is optional but recommended for better compatibility. Some videos may not play without it. For more information, see https://github.com/nalabrie/clip-to-mpv?tab=readme-ov-file#optional");
    }

    mpv_exists && yt_dlp_exists // ffmpeg is optional, so it's not included in the return value
}

// === MAIN ===

fn main() -> Result<(), Error> {
    // print app welcome message
    let version = env!("CARGO_PKG_VERSION");
    println!("clip-to-mpv version {version}");
    println!("Press Ctrl+C to exit\n");

    // check for required external apps
    if !check_for_external_apps() {
        println!("Exiting...");
        exit(1);
    }

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
