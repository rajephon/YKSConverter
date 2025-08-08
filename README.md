# YKSConverter

MML (Music Macro Language) to MIDI converter with C++ and Rust implementations.

YKSConverter converts Mabinogi MML (PSG Basic) to MIDI format. This project provides two complete implementations with 100% binary compatibility.

Implementation based on PSGConverter. [logue's PSGConverter](https://github.com/logue/PSGConverter)

## Project Structure

```
YKSConverter/
├── README.md                    # This file
├── cpp/                        # C++ implementation
│   ├── YKSConverter/           # C++ source code
│   ├── build/                  # Compiled binaries
│   ├── test_baseline.cpp       # Baseline testing tool
│   └── README.md               # C++ specific documentation
├── rust/                       # Rust implementation
│   ├── Cargo.toml              # Rust project configuration
│   ├── src/                    # Rust source code
│   └── README.md               # Rust specific documentation
├── docs/                       # Common documentation
│   ├── CLAUDE.md               # Development guidelines
│   ├── MML_SPEC.md             # MML specification
│   └── PORTING_NOTES.md        # Porting process documentation
└── scripts/                    # Testing and comparison tools
    ├── *.hex                   # Binary comparison files
    ├── *.midi                  # Sample MIDI outputs
    └── compare_outputs.sh      # Output comparison script
```

## Features

- **Complete MML Support**: Tempo, length tokens, octave changes, dotted notes
- **Multi-track MIDI**: Support for complex multi-track compositions  
- **Binary Compatibility**: C++ and Rust versions produce identical output
- **Comprehensive Testing**: Full test suite with baseline comparisons
- **Cross-platform**: Works on macOS, Linux, and Windows

## Quick Start

### C++ Version
```bash
cd cpp
mkdir build && cd build
cmake ..
make
```

### Rust Version
```bash
cd rust
cargo build --release
cargo run --bin yks_converter -- "MML@t120l4cdefgab>c4.,,;"
```

## Usage Examples

### C++ Implementation
See [cpp/README.md](cpp/README.md) for detailed usage examples including:
- Single track conversion
- Multi-track ensemble
- CMake build instructions
- Command line usage

### Rust Implementation  
See [rust/README.md](rust/README.md) for detailed usage examples including:
- Library usage (add as dependency)
- Command line tool
- Single and multi-track examples

## Development Status

- ✅ C++ to Rust port completed with 100% compatibility
- ✅ All tests passing (9/9)
- ✅ Binary compatibility verified  
- ✅ Multi-track support implemented
- ⏳ MIDI file validation with external tools (pending)

## Testing

Both implementations are thoroughly tested:

```bash
# Test Rust version
cd rust && cargo test

# Test C++ version  
cd cpp && make test

# Compare outputs
./scripts/compare_outputs.sh
```

## Contributing

See [docs/CLAUDE.md](docs/CLAUDE.md) for development guidelines and TDD practices.

## License

Copyright (c) 2018 rajephon <rajephon@gmail.com>

Licensed under BSD 2-Clause "Simplified" License

See [/LICENSE](./LICENSE) for license information.
