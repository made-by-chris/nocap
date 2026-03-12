# NoCap

<p align="center">
  <img src="icon.png" width="200" alt="NoCap logo" />
</p>

**Dead-simple screen recorder with optional webcam overlay.**

Record your screen, mic, and system audio into a single video file. No accounts, no cloud, no bloat. Just hit record.

**[Use it in your browser](https://made-by-chris.github.io/nocap/)** — no install needed.

## Features

- **Screen + audio capture** — system audio and microphone, mixed together
- **Webcam bubble** — draggable circular webcam overlay with adjustable size and position
- **Multiple output modes** — YouTube 16:9, TikTok 9:16, Square, Instagram 4:5, or Free (native resolution)
- **Layout options** — blurred background, letterbox, or fill/crop for non-matching aspect ratios
- **Seekable exports** — WebM files with proper duration metadata so scrubbing actually works
- **Tiny footprint** — ~5MB bundle (Tauri, not Electron)

## Download

Grab the latest release for your platform from [Releases](../../releases).

| Platform | File |
|----------|------|
| Windows (installer) | `NoCap_x64-setup.exe` |
| Windows (MSI) | `NoCap_x64_en-US.msi` |

## Building from source

**Prerequisites:** Rust 1.77+, Node.js 18+, [Tauri CLI v2](https://v2.tauri.app/start/prerequisites/)

```bash
npm install
npm run tauri:build
```

Outputs land in `src-tauri/target/release/bundle/`.
