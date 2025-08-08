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

### Single Track Example
```cpp
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

### Multi-Track Example
```cpp
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