# YKSConverter
YKSConverter is a tool to convert Mabinogi MML (PSG Basic) to MIDI.  
Implementation based on PSGConverter. [logue's PSGConverter](https://github.com/logue/PSGConverter)

## Usage
- just copy the YKSConverter folder to your build tree and use a C++11 compiler.
- or Build library with cmake 

## Example

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

### CMake
just simply
```bash
# go to project directory
mkdir build
cd build
cmake ..
make
make install
```
Install destination: `${ProjectDirectory}/release/`

## License

Copyright (c) 2018 rajephon <rajephon@gmail.com>

Licensed under BSD 2-Clause "Simplified" License

See [/LICENSE](./LICENSE) for license information.
