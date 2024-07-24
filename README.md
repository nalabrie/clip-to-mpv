# Clip to MPV

## Description

A simple Rust CLI app to play URLs stored in the clipboard with mpv.

## Dependencies

### Required

The following dependencies are **required** and must be in your `PATH`:

- [mpv](https://mpv.io/) media player to play the URLs.
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) used by `mpv` to play URLs (not just YouTube videos).

### Optional

The following dependencies are **optional** and must be in your `PATH`:

- [ffmpeg](https://ffmpeg.org/) to convert URLs to a format that `mpv` can play. Most often used when the audio and video are separate streams.

## Installation

Not yet available on [crates.io](https://crates.io/). To install, see the [Compilation](#compilation) section.

It will be available for installation via `cargo` once a stable version is released.

## Usage

1. Run `clip-to-mpv` in your terminal.
2. Any URLs copied to your clipboard while the app is running will be played with `mpv` immediately.
3. To stop the app, press `Ctrl+C` in the terminal.

## Compilation

To compile the app, you will need to have Rust installed.

1. Clone the repository.
2. Run `cargo build --release`.
3. The compiled binary will be in the `target/release` directory.
4. Add the binary to your `PATH` or run it directly from the directory.
5. Run the binary in your terminal. See the [Usage](#usage) section for more information.

## License

This project is licensed under the [MIT License](LICENSE).
