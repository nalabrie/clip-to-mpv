use clipboard_win::{formats, get_clipboard};
use std::io::Error;
use std::process::Command;

fn main() -> Result<(), Error> {
    let result: String = get_clipboard(formats::Unicode).unwrap_or_default();

    if result == String::default() {
        return Err(Error::new(std::io::ErrorKind::Other, "Clipboard is empty"));
    }

    Command::new("mpv.exe").arg(result).output()?;

    Ok(())
}
