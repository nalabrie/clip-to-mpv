use std::io::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use clipboard_win::{get_clipboard_string, set_clipboard_string};

// ---- GLOBAL CONSTANTS ----

// how long to wait between clipboard reads
const WAIT_DURATION: Duration = Duration::from_millis(250);

// primitive "regex alternative" to cover most cases when validating URLs
const URL_VALIDATION_ARRAY: [&str; 17] = [
    "http", "www.", ".com", ".org", ".net", ".edu", ".gov", ".info", ".io", ".biz", ".pro", ".xzy",
    ".de", ".uk", ".top", ".cn", ".tk",
];

// ---- FUNCTIONS ----

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

// ---- MAIN ----

fn main() -> Result<(), Error> {
    let mut result: String;
    let mut prev_result = String::new();
    set_clipboard_string("").expect("Clearing clipboard on first run");

    loop {
        result = get_clipboard_string().unwrap_or(prev_result.clone());

        if result == prev_result || !validate_url(&result) {
            sleep(WAIT_DURATION);
            continue;
        }

        prev_result = result.clone();
        println!("Now playing: {result}");
        Command::new("mpv.exe")
            .arg("--mute")
            .arg(result)
            .spawn()
            .expect("Launch mpv with clipboard URL");
    }
}
// sample URL: https://youtu.be/9FLRHejWAo8
