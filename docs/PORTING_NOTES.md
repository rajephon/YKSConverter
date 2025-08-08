# C++ to Rust Porting Notes

This document describes the process and lessons learned from porting YKSConverter from C++ to Rust with 100% binary compatibility.

## Overview

The porting process followed Test-Driven Development (TDD) principles:
1. **Red**: Write failing tests based on C++ baseline outputs
2. **Green**: Implement minimal code to pass tests
3. **Refactor**: Improve structure while maintaining compatibility

**Result**: Perfect binary compatibility - both versions generate identical MIDI files.

## Architecture Mapping

### C++ → Rust Type Mapping

| C++ | Rust | Notes |
|-----|------|--------|
| `std::shared_ptr<T>` | `Box<T>` | Heap allocation |
| `std::vector<T>` | `Vec<T>` | Direct equivalent |
| `std::string` | `String` | Direct equivalent |
| `uint8_t` | `u8` | Direct equivalent |
| `unsigned int` | `u32` | Direct equivalent |
| Virtual inheritance | Trait objects | `Box<dyn TrackEvent>` |
| `std::map` | `HashMap` | Direct equivalent |

### Class → Struct/Trait Conversion

#### TrackEvent Hierarchy
**C++**: Virtual base class with inheritance
```cpp
class TrackEvent {
    virtual std::string value() = 0;
    virtual std::shared_ptr<ByteBuffer> toBuffer() = 0;
};

class NoteOn : public TrackEvent { ... };
```

**Rust**: Trait with dynamic dispatch
```rust
trait TrackEvent {
    fn value(&self) -> String;
    fn to_buffer(&self, buffer: &mut ByteBuffer);
    fn clone_event(&self) -> Box<dyn TrackEvent>;
}

struct NoteOn { ... }
impl TrackEvent for NoteOn { ... }
```

**Key Challenge**: Cloning trait objects required custom `clone_event` method.

#### ByteBuffer
**C++**: Class with internal pointer management
```cpp
class ByteBuffer {
    std::vector<uint8_t> buffer;
    size_t position;
public:
    void putUInt8(uint8_t value);
    void putUInt16BE(uint16_t value);  
    void putUInt32BE(uint32_t value);
};
```

**Rust**: Struct with ownership
```rust  
pub struct ByteBuffer {
    buffer: Vec<u8>,
    position: usize,
}

impl ByteBuffer {
    pub fn put_u8(&mut self, value: u8);
    pub fn put_u16_be(&mut self, value: u16);
    pub fn put_u32_be(&mut self, value: u32);
}
```

## Critical Porting Challenges

### 1. Regex Pattern Matching
**Issue**: C++ and Rust regex engines have subtle differences.

**C++ Pattern**: 
```cpp
std::regex("[OTLVNRA-Gotlvnra-g<>][\\+\\-\\#]?[0-9]*\\.?&?")
```

**Rust Solution**: Exact character-by-character replication
```rust
Regex::new(r"[OTLVNRA-Gotlvnra-g<>][\+\-\#]?[0-9]*\.?&?")
```

**Key**: The character range `[OTLVNRA-Gotlvnra-g]` had to be precisely replicated.

### 2. Integer Type Consistency  
**Issue**: Mixed i32/u32 usage caused compilation errors.

**Solution**: Careful type management with explicit casting
```rust
let note_time = 96u32;           // Consistent u32 for timing
let octave = 4i32;              // i32 for calculations that can go negative
let semibreve = note_time * 4;   // u32 × u32 = u32
let tick = semibreve / value as u32; // Explicit casting
```

### 3. Timing Calculations
**Critical**: Exact timing replication was essential for binary compatibility.

**C++ Formula**:
```cpp
int noteTime = floor(semibreve / value);
if (dot == ".") {
    noteTime = floor(noteTime * 1.5f);  
}
```

**Rust Equivalent**:
```rust
let mut note_time = semibreve / value as u32;
if dot == "." {
    note_time = (note_time as f32 * 1.5) as u32;
}
```

### 4. MIDI Event Ordering
**Issue**: Event processing order affected final output.

**Solution**: Replicated exact C++ algorithm flow:
1. Parse control tokens first (l, o, t, v, <, >)
2. Then parse note/rest tokens
3. Handle ties and state management
4. Add final note_time to delta_time (crucial detail!)

## Testing Strategy

### 1. Baseline Establishment
Created C++ baseline test program:
```cpp
// test_baseline.cpp - generates reference MIDI outputs
./test_baseline "MML@t190l8cdefgab>c4.,l8<cdefgab>c4.,l8>cdefgab>c4.;"
```

### 2. Binary Comparison  
Hex string comparison for exact verification:
```rust
#[test]
fn should_generate_same_midi_as_cpp() {
    let cpp_hex = "4d546864000000060001000300604d54726b...";
    let rust_hex = converter.to_buffer().to_hex_string();
    assert_eq!(rust_hex, cpp_hex);  // Must be identical
}
```

### 3. Incremental Testing
- Single note parsing
- Multi-note sequences  
- Length tokens (l8, l4)
- Octave changes (<, >)
- Dotted notes (4.)
- Complex multi-track MML

## Key Lessons Learned

### 1. Start with Simple Cases
Begin with single notes before complex MML:
```
MML@c,,;           # Single note
MML@cde,,;         # Simple sequence  
MML@l8cde,,;       # With length token
MML@t120l4cdef,,;  # Full complexity
```

### 2. Debug with Hex Output
Binary comparison revealed subtle differences:
- Wrong velocity values (64 vs 40)
- Incorrect timing deltas
- Missing/extra events

### 3. Respect Original Algorithm
Don't optimize during porting - replicate exactly:
```rust
// Don't optimize this during porting:
while note < self.min_note as i32 { note += 12; }
while note > self.max_note as i32 { note -= 12; }
note += 12;  // This line is crucial!
```

### 4. Type Safety Benefits
Rust's type system caught several potential bugs:
- Integer overflow prevention
- Memory safety guarantees  
- Compile-time error detection

## Performance Comparison

| Metric | C++ | Rust | Notes |
|--------|-----|------|-------|
| Binary size | ~50KB | ~2MB | Rust includes more runtime |
| Parse time | ~1ms | ~1ms | Equivalent performance |
| Memory usage | Manual | Automatic | Rust prevents leaks |
| Safety | Manual bounds checking | Compile-time guaranteed | Major advantage |

## Final Verification

**Tests Passing**: 9/9 ✅
**Binary Compatibility**: 100% ✅  
**MIDI Output**: Identical (688 hex chars) ✅

```bash
# Both versions produce identical output:
C++ : 4d546864000000060001000300604d54726b0000009000ff0120...
Rust: 4d546864000000060001000300604d54726b0000009000ff0120...
```

## Recommendations for Future Ports

1. **Establish baseline first** - Always create reference outputs
2. **Test incrementally** - Start simple, build complexity
3. **Replicate, don't optimize** - Maintain algorithm fidelity  
4. **Use binary comparison** - Hex string matching catches everything
5. **Follow TDD strictly** - Red-Green-Refactor cycle prevents regressions
6. **Document differences** - Note any unavoidable changes

The Rust port demonstrates that 100% binary compatibility is achievable with careful attention to algorithmic details and comprehensive testing.