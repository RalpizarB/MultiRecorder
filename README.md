# MultiRecorder

A modern, cross-platform multi-track audio recorder with a friendly GUI built in Rust.

```
┌─────────────────────────────────────────────────────┐
│          🎙️  MultiRecorder                         │
│    Modern Multi-Track Audio Recorder                │
├─────────────────────────────────────────────────────┤
│                                                      │
│  Input Device:  [Default Audio Input      ▼]        │
│  Output File:   [recording_20231202_143022.wav]     │
│                                                      │
│                 [🔄 Refresh Devices]                │
│                                                      │
├─────────────────────────────────────────────────────┤
│                                                      │
│    [  ⏺ Start Recording  ]     ⏱ Duration: 0.0s    │
│                                                      │
├─────────────────────────────────────────────────────┤
│  Status: Ready to record                            │
├─────────────────────────────────────────────────────┤
│  ℹ️ Information                                      │
│    Features:                                        │
│      • Record audio from any input device           │
│      • High-quality WAV output                      │
│      • Real-time recording indicator                │
│      • Modern and intuitive interface               │
└─────────────────────────────────────────────────────┘
```

## Features

- 🎙️ Record audio from any input device
- 🎨 Modern and intuitive user interface
- 📁 High-quality WAV output format
- ⏱️ Real-time recording duration display
- 🔄 Hot-swappable audio device selection
- 🚀 Fast and lightweight

## Prerequisites

- Rust 1.70 or later
- System audio libraries (ALSA on Linux, CoreAudio on macOS, WASAPI on Windows)

### Linux
```bash
sudo apt-get install libasound2-dev pkg-config
```

### macOS
No additional dependencies required.

### Windows
No additional dependencies required.

## Installation

### From Source

```bash
git clone https://github.com/RalpizarB/MultiRecorder.git
cd MultiRecorder
cargo build --release
```

The compiled binary will be available at `target/release/multirecorder`.

## Usage

Simply run the application:

```bash
cargo run --release
```

Or execute the binary directly:

```bash
./target/release/multirecorder
```

### Basic Workflow

1. Select your input device from the dropdown
2. Choose an output filename (defaults to timestamped format)
3. Click "Start Recording" to begin
4. Click "Stop Recording" when finished
5. Find your recording in the current directory

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment. Every push triggers:
- Automated building across Linux, macOS, and Windows
- Code linting and formatting checks
- Test execution

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [egui](https://github.com/emilk/egui) - Immediate mode GUI framework
- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio I/O
- [hound](https://github.com/ruuda/hound) - WAV encoding library
