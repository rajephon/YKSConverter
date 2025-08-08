# YKSConverter Rust Implementation

Modern Rust port of the MML to MIDI converter with 100% binary compatibility with the C++ version.

## Building

```bash
cargo build --release
```

## Usage

### As Library
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

### Command Line
```bash
cargo run -- "MML@t120l4cdefgab>c4.,,;"
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