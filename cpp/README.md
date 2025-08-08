# YKSConverter C++ Implementation

Original C++ implementation of the MML to MIDI converter.

## Building

### CMake (Recommended)
```bash
mkdir build && cd build
cmake ..
make
make install  # Optional: installs to ../release/
```

### Manual Compilation
```bash
g++ -std=c++11 -I. YKSConverter/*.cpp YKSConverter/MF2TT2MF/*.cpp -o yks_converter
```

## Usage

### As Library
```cpp
#include "YKSConverter/YKSConverter.h"

int main() {
    std::string mml = "MML@t120l4cdefgab>c4.,,;";
    int instrument = 1;
    
    auto converter = std::make_shared<YKSConverter>(mml, instrument);
    auto buffer = converter->toBuffer();
    
    // Save to file
    std::ofstream out("output.midi", std::ios::binary);
    for (int i = 0; i < buffer->size(); i++) {
        out << buffer->get();
    }
    out.close();
    
    return 0;
}
```

### Command Line
```bash
./yks_converter "MML@t120l4cdefgab>c4.,,;"
```

## Testing

Run baseline tests to verify output:
```bash
g++ -std=c++11 test_baseline.cpp YKSConverter/*.cpp YKSConverter/MF2TT2MF/*.cpp -o test_baseline
./test_baseline
```

## Dependencies

- C++11 compatible compiler
- CMake 3.0+ (for CMake build)
- No external libraries required

## Architecture

- **YKSConverter**: Main converter class
- **MF2TT2MF**: MML parsing and MIDI track building
- **TrackBuilder**: MIDI track construction
- **TrackEvent**: MIDI event types (NoteOn, NoteOff, etc.)
- **ByteBuffer**: Binary MIDI data handling