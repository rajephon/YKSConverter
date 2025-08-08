# YKSConverter Rust Implementation

Modern Rust port of the MML to MIDI converter with 100% binary compatibility with the C++ version.

## Building

```bash
cargo build --release
```

## Usage

### Command Line Tool

Clone and run directly:
```bash
git clone https://github.com/rajephon/YKSConverter.git
cd YKSConverter/rust
cargo run --bin yks_converter -- "MML@t120l4cdefg,,;"
```

This generates `output.midi` in the current directory.

More examples:
```bash
# Simple melody
cargo run --bin yks_converter -- "MML@t120l4cdefg,,;"

# Complex multi-track piece  
cargo run --bin yks_converter -- "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;"
```

### Library Usage

### Single Track Example
```rust
use yks_converter::YksConverter;

fn main() {
    let mml = "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;";
    let converter = YksConverter::new(mml.to_string(), 1);
    
    if let Some(buffer) = converter.to_buffer() {
        std::fs::write("output.midi", buffer.as_slice()).unwrap();
    }
}
```

### Multi-Track Example
```rust
use yks_converter::YksConverter;

fn main() {
    let mml_tracks = vec![
        "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;".to_string(),
        "MML@t180l8>ccccccc4,l8>eeeeeee4,l8>ggggggg4;".to_string(),
    ];
    let instruments = vec![26, 74]; // MIDI instrument codes
    
    let converter = YksConverter::new_multi(mml_tracks, instruments);
    if let Some(buffer) = converter.to_buffer() {
        std::fs::write("ensemble.midi", buffer.as_slice()).unwrap();
    }
}
```

### Adding to Your Project

Add to your `Cargo.toml`:
```toml
[dependencies]
yks_converter = "0.1.0"
```

Or use cargo:
```bash
cargo add yks_converter
```

## Testing

Run the comprehensive test suite:
```bash
cargo test                           # All tests
cargo test -- --nocapture          # With debug output
cargo test binary_compatibility    # Binary compatibility test
```

### Test Coverage
- ✅ MML regex parsing
- ✅ Single note parsing  
- ✅ Multi-note sequences
- ✅ Length tokens (l8, l4, etc.)
- ✅ Octave changes (<, >)
- ✅ Dotted notes (4.)
- ✅ Tempo changes (t190)
- ✅ Multi-track support
- ✅ **100% Binary compatibility with C++**

## Dependencies

- `regex` - MML pattern matching
- `byteorder` - MIDI binary format handling

## Architecture

This Rust implementation mirrors the C++ architecture:

- **YksConverter**: Main converter struct
- **Mf2tt2mf**: MML parsing and MIDI track building  
- **TrackBuilder**: MIDI track construction
- **TrackEvent**: Trait for MIDI event types
  - MetaText, Tempo, SysEx
  - ProgramChange, ControlChange  
  - NoteOn, NoteOff, EndOfTrack
- **ByteBuffer**: Binary MIDI data handling with big-endian support

## Binary Compatibility

The Rust version produces **identical** MIDI output to the C++ version:
- Same timing calculations (384 ticks per whole note)
- Same velocity values (8 * volume)
- Same MIDI event encoding
- Verified with comprehensive binary comparison tests

## Performance

```bash
cargo bench                  # Run benchmarks
cargo build --release       # Optimized build
```