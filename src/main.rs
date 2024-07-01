use std::io::Error;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use arboard::Clipboard;
use ctrlc;

// === GLOBAL CONSTANTS ===

// how long to wait between clipboard reads
const WAIT_DURATION: Duration = Duration::from_millis(250);

// primitive "regex alternative" to cover most cases when validating URLs
// TODO: improve URL validation
const URL_VALIDATION_ARRAY: [&str; 17] = [
    "http", "www.", ".com", ".org", ".net", ".edu", ".gov", ".info", ".io", ".biz", ".pro", ".xzy",
    ".de", ".uk", ".top", ".cn", ".tk",
];

// mpv command & args
const MPV_COMMAND: &str = "mpv.exe"; // TODO: set this conditionally based on OS
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
    URL_VALIDATION_ARRAY.iter().any(|&s| url.contains(s))
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
