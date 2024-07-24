# Clip to MPV

## Description

A simple Rust CLI app to play URLs stored in the clipboard with mpv. Works and tested on Windows and Linux. Should work on macOS as well, but not yet tested.

## Dependencies

### Required

The following dependencies are **required** and must be in your `PATH`:

- [mpv](https://mpv.io/) media player to play the URLs.
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) used by `mpv` to play URLs (not just YouTube videos).

### Optional

The following dependencies are **optional** and must be in your `PATH`:

- [ffmpeg](https://ffmpeg.org/) to convert URLs to a format that `mpv` can play. Most often used when the audio and video are separate streams. In the case of YouTube videos, the highest quality audio and video streams are (often) downloaded separately and then merged by `ffmpeg` before being played by `mpv`. If `ffmpeg` is not installed, `mpv` will play a lower quality version of the video with the audio and video streams pre-merged. But some other websites may not work at all without `ffmpeg`. Your mileage may vary.

## Installation

Not yet available on [crates.io](https://crates.io/). To install, see the [Compilation](#compilation) section.

It will be available for installation via `cargo` once a stable version is released.

## Usage

1. Run `clip-to-mpv` in your terminal.
2. Any URLs copied to your clipboard while the app is running will be played with `mpv` immediately.
   - If the URL does not go to a page with media, `mpv` will simply not open and the app will continue to run.
3. To stop the app, press `Ctrl+C` in the terminal.
   - Closing the terminal will also stop the app, but your clipboard will not be cleared.

## Compilation

To compile the app, you will need to have Rust installed.

1. Clone the repository.
2. Run `cargo build --release`.
3. The compiled binary will be in the `target/release` directory.
4. Add the binary to your `PATH` or run it directly from the directory.
5. Run the binary in your terminal. See the [Usage](#usage) section for more information.

## License

This project is licensed under the [MIT License](LICENSE).
