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
cargo run -- "MML@t120l4cdefgab>c4.,,;"
```

## Examples

### MML for single
```C++
#include <iostream>
#include <memory>
#include <fstream>
#include <inttypes.h>
#include "YKSConverter/YKSConverter.h"

int main(int argc, const char * argv[]) {
    std::string txtMML = "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;";
    int inst = 1; // instrument code.
    auto yksConverter = std::make_shared<YKSConverter>(txtMML, inst);
    auto buffer = yksConverter->toBuffer();
    std::ofstream out("./output.midi");
    for (int i = 0; i < buffer->size(); i++) {
        out << buffer->get();
    }
    out.close();
    return 0;
}
```

### MML for ensemble
```C++
#include <iostream>
#include <memory>
#include <fstream>
#include <inttypes.h>
#include "YKSConverter/YKSConverter.h"

int main(int argc, const char * argv[]) {
    std::vector<std::string> mml = {
        "MML@t180l8ccccccc4,l8eeeeeee4,l8ggggggg4;",
        "MML@t180l8>ccccccc4,l8>eeeeeee4,l8>ggggggg4;"
    };
    std::vector<uint8_t> inst = {26, 74};
    auto yksConverter = std::make_shared<YKSConverter>(mml, inst);
    auto buffer = yksConverter->toBuffer();
    std::ofstream out("./output.midi");
    for (int i = 0; i < buffer->size(); i++) {
        out << buffer->get();
    }
    out.close();
    return 0;
}
```

### Rust Examples

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

### CMake (C++)
```bash
cd cpp
mkdir build && cd build
cmake ..
make
make install
```
Install destination: `${ProjectDirectory}/release/`

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
